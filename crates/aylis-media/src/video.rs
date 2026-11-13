//! Vídeo em três camadas:
//!
//! 1. **Quadros prontos de qualquer thread** ([`FrameSink`]): o seu codec, uma câmera, um
//!    stream, o FFmpeg chamado por você — entrega quadros e o `VideoPlayer` desenha.
//! 2. **Decodificador plugável** ([`VideoDecoder`]): o player cuida da thread, do relógio,
//!    de pausar, buscar e repetir.
//! 3. **Quem abre cada arquivo** ([`register_decoder`]): fábricas tentadas em ordem (as
//!    registradas por você primeiro, depois a do sistema — Media Foundation no Windows).

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use aylis_core::{Canvas, GpuDevice, GpuSurface, Image, Poster, RectF, SizeF};

use crate::audio::AudioOpener;

/// Medidas de um vídeo.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    /// Em segundos, se conhecida.
    pub duration: Option<f64>,
    /// Quadros por segundo, se conhecido.
    pub fps: Option<f64>,
}

/// Um quadro: a imagem e o tempo (em segundos) em que ele aparece.
///
/// O quadro pode estar na placa de vídeo ([`gpu`](Self::gpu), o relatório do Tabernáculo,
/// W15): aí `image` fica vazia e quem desenha usa [`draw`](Self::draw), que compõe a
/// textura sem passar pela CPU quando o canvas sabe (o Direct2D na mesma placa) e, se não,
/// copia os pixels uma vez ([`to_image`](Self::to_image)): o desenho sai igual nos três
/// renderizadores.
#[derive(Clone, Debug)]
pub struct Frame {
    pub time: f64,
    pub image: Image,
    /// O quadro como textura na placa de vídeo (`None`: os pixels estão em `image`).
    pub gpu: Option<GpuFrame>,
}

impl Frame {
    /// Quadro com pixels RGBA.
    pub fn rgba(width: u32, height: u32, pixels: Vec<u8>, time: f64) -> Self {
        Self { time, image: Image::rgba(width, height, pixels), gpu: None }
    }

    /// Quadro com pixels BGRA (o formato que os decodificadores costumam entregar).
    pub fn bgra(width: u32, height: u32, pixels: Vec<u8>, time: f64) -> Self {
        Self { time, image: Image::bgra(width, height, pixels), gpu: None }
    }

    /// Quadro que está na placa de vídeo (uma textura compartilhada; veja [`GpuFrame`]).
    pub fn texture(gpu: GpuFrame, time: f64) -> Self {
        Self { time, image: Image::bgra(0, 0, Vec::new()), gpu: Some(gpu) }
    }

    /// Tamanho em pixels.
    pub fn size(&self) -> Option<SizeF> {
        match &self.gpu {
            Some(g) => {
                let (w, h) = g.size();
                Some(SizeF::new(w as f32, h as f32))
            }
            None => self.image.known_size(),
        }
    }

    /// A imagem com os pixels: a própria `image` ou, na placa de vídeo, a cópia para a CPU
    /// (feita uma vez por quadro). `None` se a cópia falhar.
    pub fn to_image(&self) -> Option<Image> {
        match &self.gpu {
            Some(g) => g.to_image(),
            None => Some(self.image.clone()),
        }
    }

    /// Desenha a parte `src` do quadro (em pixels dele) em `dest`, como
    /// [`Canvas::draw_image`]. A textura vai direto quando o canvas aceita
    /// ([`Canvas::draw_gpu_surface`]); senão, os pixels copiados para a CPU.
    pub fn draw(&self, canvas: &mut dyn Canvas, src: RectF, dest: RectF, opacity: f32) {
        let Some(g) = &self.gpu else {
            canvas.draw_image(&self.image, src, dest, opacity);
            return;
        };
        if g.draw(canvas, src, dest, opacity) {
            return;
        }
        if let Some(image) = g.to_image() {
            canvas.draw_image(&image, src, dest, opacity);
        }
    }
}

/// Quem está por trás de um [`GpuFrame`]: o decodificador que guarda a textura.
pub trait GpuFrameSource: Send + Sync {
    /// O canvas compôs a textura (a placa dele vai lê-la): quem a reaproveita espera a cerca
    /// de liberação. Padrão: nada.
    fn drawn(&self) {}
    /// Copia a textura para a CPU, em BGRA (o caminho de reserva: Skia, navegador, o app que
    /// quer os pixels).
    fn download(&self) -> Option<Image>;
}

/// Um quadro de vídeo que está na placa (W15): a textura compartilhada ([`GpuSurface`]) que
/// o canvas compõe sem passar pela CPU, e a cópia para a CPU quando o canvas não sabe.
///
/// Enquanto houver um clone, a textura não é reescrita pelo decodificador.
#[derive(Clone)]
pub struct GpuFrame {
    pub surface: GpuSurface,
    source: Arc<dyn GpuFrameSource>,
    cpu: Arc<std::sync::OnceLock<Option<Image>>>,
}

impl std::fmt::Debug for GpuFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpuFrame").field("surface", &self.surface).finish_non_exhaustive()
    }
}

impl GpuFrame {
    pub fn new(surface: GpuSurface, source: impl GpuFrameSource + 'static) -> Self {
        Self { surface, source: Arc::new(source), cpu: Arc::default() }
    }

    /// Largura e altura da textura.
    pub fn size(&self) -> (u32, u32) {
        match self.surface {
            GpuSurface::D3D12 { width, height, .. } | GpuSurface::GlTexture { width, height, .. } | GpuSurface::WebCanvas { width, height, .. } => (width, height),
        }
    }

    /// Os pixels, copiados da placa uma vez (as próximas chamadas devolvem a mesma imagem).
    pub fn to_image(&self) -> Option<Image> {
        self.cpu.get_or_init(|| self.source.download()).clone()
    }

    /// Compõe a textura direto, se o canvas souber; `false`: quem chama usa os pixels.
    pub fn draw(&self, canvas: &mut dyn Canvas, src: RectF, dest: RectF, opacity: f32) -> bool {
        let (w, h) = self.size();
        if opacity <= 0.0 {
            return true;
        }
        if w == 0 || h == 0 || src.width <= 0.0 || src.height <= 0.0 {
            return false;
        }
        // A composição desenha a textura inteira: a parte `src` vira um destino maior,
        // recortado em `dest`.
        let (sx, sy) = (dest.width / src.width, dest.height / src.height);
        let full = RectF::new(dest.x - src.x * sx, dest.y - src.y * sy, w as f32 * sx, h as f32 * sy);
        let partial = src != RectF::new(0.0, 0.0, w as f32, h as f32);
        canvas.save();
        if partial {
            canvas.clip_rect(dest);
        }
        if opacity < 1.0 {
            canvas.save_layer(opacity);
        }
        let ok = canvas.draw_gpu_surface(full, &self.surface);
        if opacity < 1.0 {
            canvas.restore();
        }
        canvas.restore();
        if ok {
            self.source.drawn();
        }
        ok
    }
}

/// Entrega o quadro mais recente no tamanho cheio (veja [`VideoDecoder::full_frame`]); pode
/// ser chamado de qualquer thread.
pub type FullFrame = Arc<dyn Fn() -> Option<Frame> + Send + Sync>;

/// Um decodificador de vídeo. Roda numa thread do player (precisa ser `Send`).
/// Vive na thread de vídeo do player: é aberto lá ([`VideoDecoderFactory::open`]) e não
/// troca de thread (os decodificadores do sistema, como o Media Foundation, não podem).
pub trait VideoDecoder {
    fn info(&self) -> VideoInfo;
    /// Vai para `seconds` (o próximo quadro é o de lá).
    fn seek(&mut self, seconds: f64);
    /// O próximo quadro (`None` no fim).
    fn next_frame(&mut self) -> Option<Frame>;
    /// O vídeo aparece com `width` x `height` pixels: o decodificador pode entregar os
    /// quadros já nesse tamanho (um 4K num player de 1.000 pontos não precisa ser copiado
    /// inteiro da placa a cada quadro). Padrão: nada.
    fn set_output_size(&mut self, width: u32, height: u32) {
        let _ = (width, height);
    }
    /// Quem entrega o quadro mais recente no tamanho cheio (a foto da câmera que mostra os
    /// quadros reduzidos ao tamanho da tela). Padrão: nenhum (os quadros já vêm cheios).
    fn full_frame(&self) -> Option<FullFrame> {
        None
    }
    /// Vai para o quadro-chave de `seconds` (ou o anterior), sem decodificar até o ponto
    /// exato: o player muito atrasado (velocidade alta) mostra os quadros-chave. Padrão:
    /// o mesmo que [`seek`](Self::seek).
    fn seek_keyframe(&mut self, seconds: f64) {
        self.seek(seconds);
    }
    /// O player está atrasado: os quadros antes de `seconds` não vão aparecer, e o
    /// decodificador pode decodificá-los sem converter (a conversão para a tela é a parte
    /// cara). Padrão: nada (o player descarta os atrasados).
    fn skip_until(&mut self, seconds: f64) {
        let _ = seconds;
    }
    /// O canvas que mostra o vídeo desenha nesta placa (`None`: na CPU, ou o app quer os
    /// pixels): o decodificador pode entregar os quadros como textura ([`Frame::texture`]),
    /// que o canvas compõe sem a ida e volta pela CPU (W15). Padrão: nada (quadros na CPU).
    fn set_gpu_device(&mut self, device: Option<GpuDevice>) {
        let _ = device;
    }
    /// Quantos quadros, no máximo, podem esperar na fila (as texturas vêm de um conjunto
    /// fixo). Padrão: sem limite próprio.
    fn max_queued(&self) -> Option<usize> {
        None
    }
    /// A trilha de áudio, se houver (o player toca junto): abre na thread do áudio.
    fn audio(&mut self) -> Option<AudioOpener> {
        None
    }
    /// Câmera: os tamanhos que ela entrega e se tem flash (`None`: não é câmera ou o
    /// sistema não diz).
    fn camera_caps(&self) -> Option<crate::camera::CameraCaps> {
        None
    }
    /// Câmera: liga ou desliga a lanterna (o flash aceso). `false` se não deu (sem flash).
    fn set_torch(&mut self, on: bool) -> bool {
        let _ = on;
        false
    }
    /// Por que parou, se foi por erro (o player mostra e dispara `on_error`).
    fn error(&self) -> Option<String> {
        None
    }
}

/// O `file` é um endereço de rede (`http://`, `https://`, `rtsp://`), e não um arquivo: o
/// decodificador do sistema abre direto, lendo aos poucos (o relatório do Tabernáculo, W11).
pub fn is_url(path: &std::path::Path) -> bool {
    let s = path.to_string_lossy();
    let lower = s.get(..8).unwrap_or(&s).to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("rtsp://")
}

/// A extensão do arquivo (minúscula), também num endereço, sem a consulta (`?token=...`) nem
/// o fragmento.
pub(crate) fn file_extension(file: &str) -> String {
    if !is_url(std::path::Path::new(file)) {
        return std::path::Path::new(file).extension().and_then(|e| e.to_str()).unwrap_or_default().to_lowercase();
    }
    let path = file.split(['?', '#']).next().unwrap_or(file);
    let name = path.rsplit('/').next().unwrap_or(path);
    name.rsplit_once('.').map(|(_, e)| e.to_ascii_lowercase()).filter(|e| e.len() <= 5).unwrap_or_default()
}

/// De onde vem o vídeo.
#[derive(Clone, Debug)]
pub enum MediaSource {
    File(PathBuf),
    Bytes { data: Arc<Vec<u8>>, extension: String },
}

impl MediaSource {
    pub fn extension(&self) -> String {
        match self {
            MediaSource::File(p) => file_extension(&p.to_string_lossy()),
            MediaSource::Bytes { extension, .. } => extension.to_lowercase(),
        }
    }
}

/// Abre vídeos (registre com [`register_decoder`]).
pub trait VideoDecoderFactory: Send + Sync {
    fn name(&self) -> &str;
    /// Esta fábrica trata arquivos com esta extensão? (padrão: todos)
    fn accepts(&self, extension: &str) -> bool {
        let _ = extension;
        true
    }
    fn open(&self, source: &MediaSource) -> Result<Box<dyn VideoDecoder>, String>;
}

static FACTORIES: Mutex<Vec<Arc<dyn VideoDecoderFactory>>> = Mutex::new(Vec::new());

/// Registra um decodificador (FFmpeg, um codec próprio...). Os registrados por último são
/// tentados primeiro, antes do decodificador do sistema.
pub fn register_decoder(factory: impl VideoDecoderFactory + 'static) {
    if let Ok(mut list) = FACTORIES.lock() {
        list.insert(0, Arc::new(factory));
    }
}

/// Quantos decodificadores o app registrou.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(crate) fn registered_decoders() -> usize {
    FACTORIES.lock().map_or(0, |l| l.len())
}

/// O decodificador do sistema (Media Foundation no Windows).
fn system_factory() -> Option<Arc<dyn VideoDecoderFactory>> {
    #[cfg(windows)]
    {
        Some(Arc::new(crate::mf::MediaFoundation))
    }
    #[cfg(target_os = "android")]
    {
        Some(Arc::new(crate::mediacodec::MediaCodec))
    }
    #[cfg(target_os = "linux")]
    {
        Some(Arc::new(crate::gst::GStreamer))
    }
    #[cfg(target_vendor = "apple")]
    {
        Some(Arc::new(crate::avf::AvFoundation))
    }
    #[cfg(not(any(windows, target_os = "android", target_os = "linux", target_vendor = "apple")))]
    {
        None
    }
}

/// Abre um vídeo com a primeira fábrica que aceitar.
pub fn open_video(source: &MediaSource) -> Result<Box<dyn VideoDecoder>, String> {
    let ext = source.extension();
    let mut factories: Vec<Arc<dyn VideoDecoderFactory>> = FACTORIES.lock().map(|l| l.clone()).unwrap_or_default();
    factories.extend(system_factory());
    let mut errors = Vec::new();
    for f in factories.iter().filter(|f| f.accepts(&ext)) {
        match f.open(source) {
            Ok(d) => return Ok(d),
            Err(e) => errors.push(format!("{}: {e}", f.name())),
        }
    }
    Err(if errors.is_empty() { format!("nenhum decodificador para .{ext}") } else { errors.join("; ") })
}

// ---------------------------------------------------------------------------
// Quadros compartilhados entre a thread de decodificação (ou quem empurra) e a tela

#[derive(Default)]
pub(crate) struct Frames {
    pub(crate) queue: VecDeque<Frame>,
    pub(crate) info: VideoInfo,
    /// O decodificador chegou ao fim.
    pub(crate) eof: bool,
    pub(crate) error: Option<String>,
    /// Modo "ao vivo" (FrameSink): mostra o último quadro empurrado, sem relógio.
    pub(crate) live: bool,
    /// Onde o relógio do player está: o decodificador atrasado pula até lá.
    pub(crate) target: f64,
    /// O tamanho em que o vídeo aparece, em pixels (0: ainda não pintou).
    pub(crate) want: (u32, u32),
    /// Quantos quadros o decodificador prepara à frente (0: o padrão, 6).
    pub(crate) ahead: usize,
    /// A placa do canvas que mostra o vídeo (a pintura atualiza): o decodificador entrega
    /// texturas para ela (W15).
    pub(crate) gpu: Option<GpuDevice>,
    /// O app pediu os pixels ([`VideoPlayer::current_frame`](crate::VideoPlayer::current_frame))
    /// ou o canvas recusou a textura: quadros na CPU daqui em diante.
    pub(crate) cpu_only: bool,
    /// Já foi pedido à tela que repinte o controle (um aviso até a próxima pintura, e não um
    /// por quadro: 15 câmeras a 15 quadros por segundo mandavam ~950 mensagens por segundo).
    pub(crate) repaint_pending: bool,
}

/// Avisa a tela de um quadro novo em `frames`: marca só o controle `id` (a área dele, na
/// janela dele), e no máximo um aviso fica pendente até ele ser atendido. Antes, cada quadro
/// pedia a repintura inteira de todas as janelas (`request_repaint`).
pub(crate) fn notify_frame(frames: &Arc<Mutex<Frames>>, poster: &Poster, id: aylis_core::ComponentId) {
    let post = frames.lock().map_or(true, |mut f| !std::mem::replace(&mut f.repaint_pending, true));
    if !post {
        return;
    }
    let frames = frames.clone();
    poster.post(move |ui| {
        // Antes de marcar: um quadro que chegar daqui em diante pede de novo.
        if let Ok(mut f) = frames.lock() {
            f.repaint_pending = false;
        }
        if ui.contains(id) {
            ui.damage(id);
        }
    });
}

/// Entrega quadros prontos ao `VideoPlayer`, de qualquer thread (câmera, rede, um codec
/// seu). O player mostra sempre o último.
#[derive(Clone)]
pub struct FrameSink {
    pub(crate) frames: Arc<Mutex<Frames>>,
    pub(crate) poster: Poster,
    /// O player: só a área dele é repintada.
    pub(crate) id: aylis_core::ComponentId,
}

impl FrameSink {
    pub fn push_frame(&self, frame: Frame) {
        if let Ok(mut f) = self.frames.lock() {
            f.live = true;
            f.queue.clear();
            if let Some((w, h)) = frame.size().map(|s| (s.width as u32, s.height as u32)) {
                f.info.width = w;
                f.info.height = h;
            }
            f.queue.push_back(frame);
        }
        notify_frame(&self.frames, &self.poster, self.id);
    }
}

// ---------------------------------------------------------------------------
// A thread de decodificação

#[derive(Default)]
pub(crate) struct DecodeControl {
    pub(crate) seek: Option<f64>,
    pub(crate) quit: bool,
}

pub(crate) struct DecodeThread {
    pub(crate) control: Arc<Mutex<DecodeControl>>,
    thread: Option<JoinHandle<()>>,
}

impl DecodeThread {
    /// Abre o vídeo numa thread própria (o decodificador nasce e vive nela) e decodifica à
    /// frente (até alguns quadros na fila). Devolve também a trilha de áudio, se houver.
    pub(crate) fn open(source: MediaSource, frames: Arc<Mutex<Frames>>, poster: Poster) -> Result<(Self, Option<AudioOpener>), String> {
        let control = Arc::new(Mutex::new(DecodeControl::default()));
        let thread_control = control.clone();
        let (opened_tx, opened_rx) = std::sync::mpsc::channel::<Result<Option<AudioOpener>, String>>();
        let thread = std::thread::Builder::new()
            .name("aylis-video".into())
            .spawn(move || {
                let mut decoder = match open_video(&source) {
                    Ok(d) => d,
                    Err(e) => {
                        let _ = opened_tx.send(Err(e));
                        return;
                    }
                };
                if let Ok(mut f) = frames.lock() {
                    f.info = decoder.info();
                    f.live = false;
                }
                let _ = opened_tx.send(Ok(decoder.audio()));
                // Quantos quadros à frente: o do player (`frames_ahead`), ou 6.
                let ahead_of = |f: &Frames| if f.ahead > 0 { f.ahead } else { 6 };
                // O último quadro decodificado (para saber se está atrás do relógio).
                let mut last = f64::NEG_INFINITY;
                // O tamanho já pedido ao decodificador.
                let mut applied = (0u32, 0u32);
                // A placa já avisada ao decodificador (W15).
                let mut applied_gpu: Option<GpuDevice> = None;
                loop {
                    let (quit, seek) = match thread_control.lock() {
                        Ok(mut c) => (c.quit, c.seek.take()),
                        Err(_) => break,
                    };
                    if quit {
                        break;
                    }
                    if let Some(t) = seek {
                        decoder.seek(t);
                        last = t;
                        if let Ok(mut f) = frames.lock() {
                            f.queue.clear();
                            f.eof = false;
                            f.target = t;
                        }
                    }
                    // Atrás do relógio (vídeo pesado, velocidade alta): muito atrás, salta
                    // para o quadro-chave de onde ele está e mostra esse (decodificar até o
                    // ponto exato demora mais que o relógio anda); pouco, decodifica sem
                    // converter até lá. Passou do fim: acabou.
                    let (target, frame_time, duration) = frames.lock().map_or((0.0, 1.0 / 30.0, None), |f| (f.target, 1.0 / f.info.fps.unwrap_or(30.0).max(1.0), f.info.duration));
                    if last.is_finite() && duration.is_some_and(|d| target >= d) {
                        if let Ok(mut f) = frames.lock() {
                            f.eof = true;
                        }
                    } else if last.is_finite() && target - last > 1.0 {
                        decoder.seek_keyframe(target);
                        last = target;
                        if let Ok(mut f) = frames.lock() {
                            f.queue.clear();
                        }
                    } else if last.is_finite() && target - last > frame_time * 2.0 {
                        decoder.skip_until(target);
                    }
                    // O tamanho na tela mudou bastante (mais de 10%): pede quadros nesse tamanho.
                    let want = frames.lock().map_or((0, 0), |f| f.want);
                    if want.0 > 0 && want.1 > 0 && (want.0 as f32 - applied.0 as f32).abs() > applied.0 as f32 * 0.1 + 1.0 {
                        decoder.set_output_size(want.0, want.1);
                        applied = want;
                    }
                    // A placa do canvas mudou (ou o app quer os pixels): o decodificador troca
                    // entre texturas e pixels.
                    let gpu = frames.lock().ok().and_then(|f| f.gpu.filter(|_| !f.cpu_only));
                    if gpu != applied_gpu {
                        decoder.set_gpu_device(gpu);
                        applied_gpu = gpu;
                    }
                    let limit = decoder.max_queued().unwrap_or(usize::MAX).max(1);
                    let (full, eof) = frames.lock().map_or((true, true), |f| (f.queue.len() >= ahead_of(&f).min(limit), f.eof));
                    if full || eof {
                        // Fila cheia: 6 quadros (200 ms a 30 fps) dão folga para esperar um
                        // pouco. No fim do arquivo (ou pausado no fim), quase nada a fazer.
                        std::thread::sleep(Duration::from_millis(if eof { 50 } else { 12 }));
                        continue;
                    }
                    let frame = decoder.next_frame();
                    let first = if let Ok(mut f) = frames.lock() {
                        let first = f.queue.is_empty();
                        // O decodificador abre o arquivo aqui (na thread dele): as medidas chegam agora.
                        f.info = decoder.info();
                        match frame {
                            Some(frame) => {
                                last = frame.time;
                                f.queue.push_back(frame)
                            }
                            None => {
                                f.eof = true;
                                f.error = decoder.error();
                            }
                        }
                        first
                    } else {
                        false
                    };
                    // O primeiro quadro (depois de abrir ou buscar) já pode aparecer.
                    if first {
                        poster.wake();
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        let mut this = Self { control, thread: Some(thread) };
        match opened_rx.recv() {
            Ok(Ok(audio)) => Ok((this, audio)),
            Ok(Err(e)) => Err(e),
            Err(_) => {
                this.thread.take().map(JoinHandle::join);
                Err("o decodificador de vídeo parou ao abrir".into())
            }
        }
    }

    pub(crate) fn seek(&self, seconds: f64) {
        if let Ok(mut c) = self.control.lock() {
            c.seek = Some(seconds.max(0.0));
        }
    }
}

impl Drop for DecodeThread {
    /// Avisa a thread e não espera (como no áudio): um `next_frame` preso numa fonte de rede
    /// congelaria a interface. A thread sai no próximo laço e solta o decodificador.
    fn drop(&mut self) {
        if let Ok(mut c) = self.control.lock() {
            c.quit = true;
        }
        drop(self.thread.take());
    }
}
