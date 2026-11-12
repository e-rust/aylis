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

use aylis_core::{Image, Poster};

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
#[derive(Clone, Debug)]
pub struct Frame {
    pub time: f64,
    pub image: Image,
}

impl Frame {
    /// Quadro com pixels RGBA.
    pub fn rgba(width: u32, height: u32, pixels: Vec<u8>, time: f64) -> Self {
        Self { time, image: Image::rgba(width, height, pixels) }
    }

    /// Quadro com pixels BGRA (o formato que os decodificadores costumam entregar).
    pub fn bgra(width: u32, height: u32, pixels: Vec<u8>, time: f64) -> Self {
        Self { time, image: Image::bgra(width, height, pixels) }
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
#[cfg_attr(any(target_arch = "wasm32", target_os = "android"), allow(dead_code))]
pub fn is_url(path: &std::path::Path) -> bool {
    let s = path.to_string_lossy();
    let lower = s.get(..8).unwrap_or(&s).to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("rtsp://")
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
            MediaSource::File(p) => p.extension().and_then(|e| e.to_str()).unwrap_or_default().to_lowercase(),
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
}

/// Entrega quadros prontos ao `VideoPlayer`, de qualquer thread (câmera, rede, um codec
/// seu). O player mostra sempre o último.
#[derive(Clone)]
pub struct FrameSink {
    pub(crate) frames: Arc<Mutex<Frames>>,
    pub(crate) poster: Poster,
}

impl FrameSink {
    pub fn push_frame(&self, frame: Frame) {
        if let Ok(mut f) = self.frames.lock() {
            f.live = true;
            f.queue.clear();
            if let Some((w, h)) = frame.image.known_size().map(|s| (s.width as u32, s.height as u32)) {
                f.info.width = w;
                f.info.height = h;
            }
            f.queue.push_back(frame);
        }
        self.poster.post(|ui| ui.request_repaint());
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
                    let (full, eof) = frames.lock().map_or((true, true), |f| (f.queue.len() >= ahead_of(&f), f.eof));
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
