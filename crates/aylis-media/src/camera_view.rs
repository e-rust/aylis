//! O componente [`CameraView`]: a imagem ao vivo da câmera, desenhada pelo próprio aylis
//! (cantos, controles por cima), com foto em PNG.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use aylis_core::{Brush, Canvas, Color, Component, ComponentId, ControlBase, EventArgs, EventCtx, ImageFit, ImageSource, PaintCtx, RectF, TextAlign, Ui, properties};

use crate::camera::{CameraCaps, CameraQuality, CameraRequest};
use crate::video::{Frame, Frames};

/// A câmera ao vivo. `active` liga; `camera` escolhe (vazio: a primeira; `front` ou `back`:
/// a da tela ou a de trás, no celular; ou o `id` de `cameras()`, que
/// lista); `mirror` espelha (a câmera frontal costuma parecer mais natural assim).
///
/// A qualidade é igual em qualquer aparelho: `quality` (`low` até 640x480, `medium` até
/// 1280x960, `high` até 1920x1440, `max` o maior) pega o maior tamanho do aparelho até o
/// limite, na proporção do sensor; `resolution` (`1920x1080`) pede um tamanho exato, se a
/// câmera tiver. Os limites do aparelho, depois de abrir (`on_ready`): `sizes` (os tamanhos
/// que a câmera entrega, do maior para o menor), `frame_size` (o de agora) e `has_flash`;
/// `torch` acende o flash como lanterna. A qualidade vale no Android, no Windows, no Linux
/// (V4L2) e no navegador (o ideal do `getUserMedia`); flash, no celular e no navegador que deixa.
pub struct CameraView {
    pub base: ControlBase,
    /// Qual câmera: vazio, a primeira; `front` ou `back`, a da tela ou a de trás (celular); ou
    /// o `id` de uma da lista de `cameras()`.
    pub camera: String,
    /// Liga a câmera e mostra a imagem ao vivo; desligado, a câmera fecha.
    pub active: bool,
    /// Espelha a imagem na horizontal (a frontal costuma parecer mais natural assim).
    pub mirror: bool,
    /// Como a imagem ocupa o controle: `contain` (inteira), `cover` (cobre, cortando as
    /// bordas), `stretch`, `center` ou `original`.
    pub fit: ImageFit,
    /// O raio dos cantos arredondados, em pontos (0: retos).
    pub radius: f32,
    /// O tamanho pedido: `low` (até 640x480), `medium` (até 1280x960), `high` (até
    /// 1920x1440) ou `max` (o maior do aparelho).
    pub quality: CameraQuality,
    /// Um tamanho exato (`1920x1080`), se a câmera tiver; vazio: vale a `quality`.
    pub resolution: String,
    /// Acende o flash como lanterna (onde houver: veja `has_flash`).
    pub torch: bool,
    /// O que a câmera aberta oferece (a thread da câmera preenche ao abrir).
    caps: Arc<Mutex<Option<CameraCaps>>>,
    /// O `torch` para a thread da câmera (ela aplica quando muda).
    torch_on: Arc<AtomicBool>,
    frames: Arc<Mutex<Frames>>,
    /// O quadro cheio para a foto (a câmera que mostra os quadros reduzidos à tela).
    full: Arc<Mutex<Option<crate::video::FullFrame>>>,
    quit: Arc<AtomicBool>,
    error: Arc<Mutex<Option<String>>>,
    /// A thread da câmera aberta: a próxima espera ela fechar (muitos celulares não abrem
    /// duas câmeras ao mesmo tempo; sem isso, trocar de câmera congelava a imagem).
    #[cfg(not(target_arch = "wasm32"))]
    worker: Option<std::thread::JoinHandle<()>>,
    #[cfg(target_arch = "wasm32")]
    web: Option<crate::web::WebVideo>,
}

impl Default for CameraView {
    fn default() -> Self {
        Self {
            base: ControlBase::new(320.0, 240.0),
            camera: String::new(),
            active: false,
            mirror: false,
            fit: ImageFit::Cover,
            radius: 0.0,
            quality: CameraQuality::default(),
            resolution: String::new(),
            torch: false,
            caps: Arc::default(),
            torch_on: Arc::new(AtomicBool::new(false)),
            frames: Arc::default(),
            full: Arc::default(),
            quit: Arc::new(AtomicBool::new(true)),
            error: Arc::default(),
            #[cfg(not(target_arch = "wasm32"))]
            worker: None,
            #[cfg(target_arch = "wasm32")]
            web: None,
        }
    }
}

impl CameraView {
    /// O que a câmera aberta oferece (`None` antes de abrir ou se o sistema não diz).
    pub fn caps(&self) -> Option<CameraCaps> {
        self.caps.lock().ok()?.clone()
    }

    /// Os tamanhos da câmera aberta, do maior para o menor (`4032x3024;1920x1080;...`).
    pub fn sizes(&self) -> String {
        self.caps().map(|c| c.sizes.iter().map(|(w, h)| format!("{w}x{h}")).collect::<Vec<_>>().join(";")).unwrap_or_default()
    }

    /// O tamanho dos quadros de agora (`1280x960`; vazio antes do primeiro).
    pub fn frame_size(&self) -> String {
        // O tamanho da câmera (o da tela pode ser menor: a câmera reduz para ela).
        self.frames
            .lock()
            .ok()
            .filter(|f| !f.queue.is_empty() && f.info.width > 0)
            .map(|f| format!("{}x{}", f.info.width, f.info.height))
            .unwrap_or_default()
    }

    /// A câmera aberta tem flash (para o `torch`).
    pub fn has_flash(&self) -> bool {
        self.caps().is_some_and(|c| c.has_flash)
    }

    fn set_sizes(&mut self, _: String) {}
    fn set_frame_size(&mut self, _: String) {}
    fn set_has_flash(&mut self, _: bool) {}

    fn request(&self) -> CameraRequest {
        CameraRequest { quality: self.quality, resolution: crate::camera::parse_size(&self.resolution) }
    }

    /// O quadro mais recente (para tirar foto ou analisar).
    pub fn frame(ui: &Ui, id: ComponentId) -> Option<Frame> {
        ui.get::<CameraView>(id)?.frames.lock().ok()?.queue.back().cloned()
    }

    /// Salva o quadro atual em PNG.
    pub fn photo(ui: &Ui, id: ComponentId, path: &std::path::Path) -> Result<(), String> {
        // No tamanho cheio da câmera, mesmo com a tela mostrando os quadros reduzidos.
        let full = ui.get::<CameraView>(id).and_then(|c| c.full.lock().ok()?.clone()).and_then(|f| f());
        let frame = full.or_else(|| Self::frame(ui, id)).ok_or("a câmera ainda não mostrou imagem")?;
        let (w, h, rgba) = match frame.image.source() {
            ImageSource::Rgba { width, height, pixels } => (*width, *height, pixels.clone()),
            ImageSource::Bgra { width, height, pixels } => {
                let mut px = pixels.clone();
                for p in px.chunks_exact_mut(4) {
                    p.swap(0, 2);
                }
                (*width, *height, px)
            }
            ImageSource::Encoded(bytes) => return std::fs::write(path, bytes).map_err(|e| e.to_string()),
        };
        let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
        let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), w, h);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        // A compressão rápida: um PNG de 12 MP no celular levava 20 s na padrão.
        encoder.set_compression(png::Compression::Fast);
        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer.write_image_data(&rgba).map_err(|e| e.to_string())
    }

    fn stop(&mut self) {
        self.quit.store(true, Ordering::Relaxed);
        #[cfg(target_arch = "wasm32")]
        {
            self.web = None;
        }
        // Sem câmera, sem foto do último quadro (e os 12 MP guardados vão embora).
        if let Ok(mut full) = self.full.lock() {
            *full = None;
        }
        if let Ok(mut f) = self.frames.lock() {
            f.queue.clear();
        }
    }

    fn start(&mut self, ctx: &mut EventCtx<'_>) {
        self.stop();
        if let Ok(mut e) = self.error.lock() {
            *e = None;
        }
        if ctx.ui.design_mode() {
            return;
        }
        let poster = ctx.ui.poster();
        #[cfg(target_arch = "wasm32")]
        {
            if let Ok(mut c) = self.caps.lock() {
                *c = None;
            }
            match crate::web::WebVideo::open_camera(&self.camera, &self.request(), self.caps.clone(), self.frames.clone(), self.error.clone(), poster, ctx.id) {
                Ok(w) => self.web = Some(w),
                Err(e) => ctx.fire("on_error", EventArgs::Text(e)),
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let quit = Arc::new(AtomicBool::new(false));
            self.quit = quit.clone();
            let previous = self.worker.take();
            let (frames, error, camera, id) = (self.frames.clone(), self.error.clone(), self.camera.clone(), ctx.id);
            let (request, caps, torch, full) = (self.request(), self.caps.clone(), self.torch_on.clone(), self.full.clone());
            if let Ok(mut c) = caps.lock() {
                *c = None;
            }
            if let Ok(mut f) = full.lock() {
                *f = None;
            }
            let spawned = std::thread::Builder::new().name("aylis-camera".into()).spawn(move || {
                if let Some(previous) = previous {
                    let _ = previous.join();
                    // O último quadro da câmera anterior não fica na tela (com o espelho da nova).
                    if let Ok(mut f) = frames.lock() {
                        f.queue.clear();
                    }
                    poster.post(|ui| ui.request_repaint());
                }
                let fail = |e: String| {
                    if let Ok(mut slot) = error.lock() {
                        *slot = Some(e.clone());
                    }
                    poster.post(move |ui| {
                        ui.fire(id, "on_error", &EventArgs::Text(e));
                        ui.request_repaint();
                    });
                };
                let mut decoder = match crate::camera::open_camera_with(&camera, &request) {
                    Ok(d) => d,
                    Err(e) => return fail(e),
                };
                // A lanterna, como estava pedida; depois, a cada quadro, se mudou.
                let mut torch_now = false;
                let mut ready = false;
                if let Ok(mut f) = full.lock() {
                    *f = decoder.full_frame();
                }
                // O tamanho em que a imagem aparece: a câmera pode entregar já reduzida.
                let mut shown = (0, 0);
                while !quit.load(Ordering::Relaxed) {
                    let want = frames.lock().map_or((0, 0), |f| f.want);
                    if want != shown && want.0 > 0 {
                        decoder.set_output_size(want.0, want.1);
                        shown = want;
                    }
                    let Some(frame) = decoder.next_frame() else {
                        return fail(decoder.error().unwrap_or_else(|| "a câmera parou".into()));
                    };
                    let wanted = torch.load(Ordering::Relaxed);
                    if wanted != torch_now {
                        decoder.set_torch(wanted);
                        torch_now = wanted;
                    }
                    if !ready {
                        // O primeiro quadro: os limites do aparelho já são conhecidos.
                        ready = true;
                        if let Ok(mut c) = caps.lock() {
                            *c = decoder.camera_caps();
                        }
                        poster.post(move |ui| {
                            ui.fire(id, "on_ready", &EventArgs::None);
                        });
                    }
                    // Mandaram parar enquanto o quadro chegava: ele não entra (seria o da câmera velha).
                    if quit.load(Ordering::Relaxed) {
                        break;
                    }
                    if let Ok(mut f) = frames.lock() {
                        f.live = true;
                        // O tamanho da câmera (o `info` do decodificador), não o do quadro
                        // reduzido para a tela.
                        let info = decoder.info();
                        let (w, h) = if info.width > 0 {
                            (info.width, info.height)
                        } else {
                            frame.image.known_size().map_or((0, 0), |s| (s.width as u32, s.height as u32))
                        };
                        f.info.width = w;
                        f.info.height = h;
                        f.queue.clear();
                        f.queue.push_back(frame);
                    }
                    // Só a área desta câmera, e um aviso até a tela atender.
                    crate::video::notify_frame(&frames, &poster, id);
                }
            });
            match spawned {
                Ok(handle) => self.worker = Some(handle),
                Err(e) => ctx.fire("on_error", EventArgs::Text(e.to_string())),
            }
        }
    }
}

impl Drop for CameraView {
    fn drop(&mut self) {
        self.quit.store(true, Ordering::Relaxed);
    }
}

impl Component for CameraView {
    fn class_name(&self) -> &'static str {
        "CameraView"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        camera: String => "Behavior",
        active: bool => "Behavior",
        mirror: bool => "Appearance",
        fit: ImageFit => "Appearance",
        radius: f32 => "Appearance",
        quality: CameraQuality => "Behavior",
        resolution: String => "Behavior",
        torch: bool => "Behavior";
        events: on_error, on_ready;
        computed: sizes: String => sizes / set_sizes, frame_size: String => frame_size / set_frame_size, has_flash: bool => has_flash / set_has_flash
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "torch" {
            self.torch_on.store(self.torch, Ordering::Relaxed);
            #[cfg(target_arch = "wasm32")]
            if let Some(web) = &self.web {
                web.set_torch(self.torch);
            }
            return;
        }
        // Trocar a qualidade com a câmera ligada reabre nela.
        if matches!(name, "active" | "camera") || (matches!(name, "quality" | "resolution") && self.active) {
            if self.active {
                self.start(ctx);
            } else {
                self.stop();
            }
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        // O tamanho em pixels em que a imagem aparece: a câmera entrega já perto dele (um
        // quadro de 12 MP numa tela de 1080 px de largura é desperdício).
        if let Ok(mut f) = self.frames.lock() {
            let scale = canvas.device_scale().max(0.1);
            f.want = ((r.width * scale).round().max(0.0) as u32, (r.height * scale).round().max(0.0) as u32);
        }
        canvas.save();
        if self.radius > 0.0 {
            canvas.clip_round_rect(r, self.radius);
        } else {
            canvas.clip_rect(r);
        }
        canvas.fill_rect(r, 0.0, &Brush::Solid(ctx.colors.fill.unwrap_or(Color::rgb(0x11, 0x11, 0x11))));
        let frame = self.frames.lock().ok().and_then(|f| f.queue.back().cloned());
        match frame {
            Some(frame) => {
                if let Some(size) = canvas.image_size(&frame.image) {
                    let (src, dest) = self.fit.place(size, r);
                    if self.mirror {
                        canvas.save();
                        canvas.translate(r.x * 2.0 + r.width, 0.0);
                        canvas.scale(-1.0, 1.0);
                        canvas.draw_image(&frame.image, src, dest, 1.0);
                        canvas.restore();
                    } else {
                        canvas.draw_image(&frame.image, src, dest, 1.0);
                    }
                }
            }
            None => {
                let mut t = ctx.text_style();
                t.color = ctx.colors.text.unwrap_or(Color::WHITE.with_opacity(0.8));
                t.h_align = TextAlign::Center;
                t.wrap = true;
                let text = match self.error.lock().ok().and_then(|e| e.clone()) {
                    Some(e) => format!("⚠ {e}"),
                    None if self.active => "…".into(),
                    None => "📷".into(),
                };
                canvas.draw_text(&text, RectF::new(r.x + 12.0, r.y, r.width - 24.0, r.height), &t);
            }
        }
        canvas.restore();
    }
}
