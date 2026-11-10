//! O [`HttpService`](aylis_core::platform::HttpService): GET sem travar a interface.
//!
//! - Fora do navegador: `ureq` (rustls, sem OpenSSL) em 4 threads de download (um mapa pede
//!   dezenas de blocos de uma vez; mais que isso só sobrecarrega o servidor).
//! - No navegador: o `fetch` do próprio navegador.

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::NativeHttp;
#[cfg(target_arch = "wasm32")]
pub(crate) use web::WebHttp;

type Done = Box<dyn FnOnce(Result<Vec<u8>, String>) + Send>;

/// O User-Agent dos pedidos (servidores de mapas pedem que o app se identifique; no
/// navegador, quem manda é o próprio navegador).
#[cfg(not(target_arch = "wasm32"))]
const USER_AGENT: &str = concat!("aylis/", env!("CARGO_PKG_VERSION"), " (+https://github.com/e-rust/aylis)");

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::io::Read;
    use std::sync::mpsc::{Receiver, Sender, channel};
    use std::sync::{Arc, Mutex};

    use super::{Done, USER_AGENT};

    const WORKERS: usize = 4;
    /// Maior resposta aceita (64 MB).
    const MAX_BODY: u64 = 64 * 1024 * 1024;

    #[derive(Default)]
    pub(crate) struct NativeHttp {
        queue: Option<Sender<(String, Done)>>,
    }

    impl NativeHttp {
        fn sender(&mut self) -> &Sender<(String, Done)> {
            self.queue.get_or_insert_with(|| {
                let (tx, rx) = channel::<(String, Done)>();
                let rx = Arc::new(Mutex::new(rx));
                for i in 0..WORKERS {
                    let rx = rx.clone();
                    let _ = std::thread::Builder::new().name(format!("aylis-http-{i}")).spawn(move || worker(&rx));
                }
                tx
            })
        }
    }

    fn worker(rx: &Mutex<Receiver<(String, Done)>>) {
        let agent: ureq::Agent = ureq::Agent::config_builder().user_agent(USER_AGENT).build().into();
        loop {
            let job = match rx.lock() {
                Ok(r) => r.recv(),
                Err(_) => return,
            };
            let Ok((url, done)) = job else { return };
            done(fetch(&agent, &url));
        }
    }

    fn fetch(agent: &ureq::Agent, url: &str) -> Result<Vec<u8>, String> {
        let response = agent.get(url).call().map_err(|e| e.to_string())?;
        let mut body = Vec::new();
        response.into_body().into_reader().take(MAX_BODY).read_to_end(&mut body).map_err(|e| e.to_string())?;
        Ok(body)
    }

    impl aylis_core::platform::HttpService for NativeHttp {
        fn get(&mut self, url: &str, done: Done) {
            if let Err(e) = self.sender().send((url.to_string(), done)) {
                let (_, done) = e.0;
                done(Err("HTTP: sem threads de download".into()));
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use wasm_bindgen::JsCast;

    use super::Done;

    pub(crate) struct WebHttp;

    async fn fetch(url: String) -> Result<Vec<u8>, String> {
        let window = web_sys::window().ok_or("sem window")?;
        let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(&url)).await.map_err(|e| format!("{e:?}"))?;
        let response: web_sys::Response = response.dyn_into().map_err(|_| "resposta")?;
        if !response.ok() {
            return Err(format!("HTTP {}", response.status()));
        }
        let buffer = wasm_bindgen_futures::JsFuture::from(response.array_buffer().map_err(|e| format!("{e:?}"))?).await.map_err(|e| format!("{e:?}"))?;
        Ok(js_sys::Uint8Array::new(&buffer).to_vec())
    }

    impl aylis_core::platform::HttpService for WebHttp {
        fn get(&mut self, url: &str, done: Done) {
            let url = url.to_string();
            wasm_bindgen_futures::spawn_local(async move { done(fetch(url).await) });
        }
    }
}
