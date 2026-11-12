//! Áudio: fontes de amostras ([`AudioSource`]), decodificação com o `symphonia` e saída
//! com o `cpal` ([`AudioPlayer`]).

use std::collections::VecDeque;
use std::io::Cursor;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_NULL, Decoder, DecoderOptions};
use symphonia::core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;

use crate::output::{MIX_CHANNELS, MIX_RATE, OutputSpec, Sink, SinkQueue, Tap, Taps, drift_correction};

/// Uma fonte de amostras de áudio (um arquivo decodificado, a trilha de um vídeo, um
/// sintetizador...). Amostras `f32` intercaladas, de -1 a 1. Vive na thread do áudio: as
/// fontes do sistema (o Media Foundation) não trocam de thread; veja [`AudioOpener`].
pub trait AudioSource {
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u16;
    /// Acrescenta mais amostras em `out`. Devolve `false` no fim.
    fn read(&mut self, out: &mut Vec<f32>) -> bool;
    /// Vai para `seconds` (a próxima leitura começa ali).
    fn seek(&mut self, seconds: f64);
    fn duration(&self) -> Option<f64> {
        None
    }
}

/// Abre a fonte na thread do áudio ([`AudioPlayer::open`]).
pub type AudioOpener = Box<dyn FnOnce() -> Result<Box<dyn AudioSource>, String> + Send>;

/// Abre o som de `location`, um arquivo do disco ou um endereço (`http://`, `https://`), na
/// thread que chama (use dentro de um [`AudioOpener`]: a fonte vive na thread do áudio).
///
/// Por endereço: o Symphonia lendo por HTTP com `Range` (feature `http`, a padrão) e, no
/// Windows, o Media Foundation quando o Symphonia não abre (o WMA, ou sem a feature).
pub fn open_audio_location(location: &str) -> Result<Box<dyn AudioSource>, String> {
    if !crate::video::is_url(std::path::Path::new(location)) {
        return SymphoniaSource::open_file(std::path::Path::new(location)).map(|s| Box::new(s) as Box<dyn AudioSource>);
    }
    #[allow(unused_mut)]
    let mut errors: Vec<String> = Vec::new();
    #[cfg(all(feature = "http", not(target_arch = "wasm32")))]
    match SymphoniaSource::open_url(location) {
        Ok(s) => return Ok(Box::new(s)),
        Err(e) => errors.push(e),
    }
    #[cfg(windows)]
    match crate::mf::open_audio(location) {
        Ok(s) => return Ok(s),
        Err(e) => errors.push(e),
    }
    if errors.is_empty() {
        errors.push(format!("{location}: o som por endereço precisa da feature `http` do aylis-media"));
    }
    Err(errors.join("; "))
}

/// Arquivo de áudio decodificado com o `symphonia`: MP3, AAC/M4A, WAV, FLAC, OGG Vorbis.
pub struct SymphoniaSource {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track: u32,
    sample_rate: u32,
    channels: u16,
    duration: Option<f64>,
}

impl SymphoniaSource {
    /// Abre um arquivo já lido. `extension` ajuda a reconhecer o formato.
    pub fn open(bytes: Vec<u8>, extension: Option<&str>) -> Result<Self, String> {
        Self::from_stream(MediaSourceStream::new(Box::new(Cursor::new(bytes)), Default::default()), extension)
    }

    /// Abre um arquivo do disco, lido aos poucos enquanto toca (a trilha de um vídeo de
    /// gigabytes não vai inteira para a memória).
    pub fn open_file(path: &std::path::Path) -> Result<Self, String> {
        let file = std::fs::File::open(path).map_err(|e| format!("não consegui ler {}: {e}", path.display()))?;
        let extension = path.extension().and_then(|e| e.to_str()).map(str::to_lowercase);
        Self::from_stream(MediaSourceStream::new(Box::new(file), Default::default()), extension.as_deref())
    }

    /// Abre um endereço (`http://`, `https://`), lido aos poucos com `Range` enquanto toca
    /// (o relatório do Tabernáculo, W11). Bloqueia até o primeiro pedido responder: chame
    /// fora da thread da interface (o [`AudioPlayer::open`] abre na thread do áudio).
    #[cfg(all(feature = "http", not(target_arch = "wasm32")))]
    pub fn open_url(url: &str) -> Result<Self, String> {
        let source = crate::http::HttpSource::open(url)?;
        let extension = crate::video::file_extension(url);
        Self::from_stream(MediaSourceStream::new(Box::new(source), Default::default()), Some(extension.as_str()).filter(|e| !e.is_empty()))
    }

    fn from_stream(stream: MediaSourceStream, extension: Option<&str>) -> Result<Self, String> {
        let mut hint = Hint::new();
        if let Some(e) = extension {
            hint.with_extension(e);
        }
        let probed = symphonia::default::get_probe()
            .format(&hint, stream, &FormatOptions { enable_gapless: true, ..Default::default() }, &MetadataOptions::default())
            .map_err(|e| format!("formato de áudio não reconhecido: {e}"))?;
        let format = probed.format;
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or_else(|| aylis_core::messages::t("o arquivo não tem trilha de áudio"))?;
        let params = track.codec_params.clone();
        let decoder = symphonia::default::get_codecs()
            .make(&params, &DecoderOptions::default())
            .map_err(|e| format!("codec de áudio não suportado: {e}"))?;
        let sample_rate = params.sample_rate.unwrap_or(44_100);
        let channels = params.channels.map_or(2, |c| c.count() as u16);
        let duration = params.n_frames.map(|n| n as f64 / sample_rate as f64);
        Ok(Self { track: track.id, format, decoder, sample_rate, channels, duration })
    }
}

impl AudioSource for SymphoniaSource {
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn read(&mut self, out: &mut Vec<f32>) -> bool {
        loop {
            let packet = match self.format.next_packet() {
                Ok(p) => p,
                Err(_) => return false, // fim (ou arquivo cortado)
            };
            if packet.track_id() != self.track {
                continue;
            }
            match self.decoder.decode(&packet) {
                Ok(decoded) => {
                    let spec = *decoded.spec();
                    self.channels = spec.channels.count() as u16;
                    self.sample_rate = spec.rate;
                    let mut buffer = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
                    buffer.copy_interleaved_ref(decoded);
                    out.extend_from_slice(buffer.samples());
                    return true;
                }
                // Pacote com defeito: pula (como os players fazem).
                Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
                Err(_) => return false,
            }
        }
    }

    fn seek(&mut self, seconds: f64) {
        let to = SeekTo::Time { time: Time::from(seconds.max(0.0)), track_id: Some(self.track) };
        if self.format.seek(SeekMode::Accurate, to).is_ok() {
            self.decoder.reset();
        }
    }

    fn duration(&self) -> Option<f64> {
        self.duration
    }
}

// ---------------------------------------------------------------------------
// Conversão de taxa e de canais

/// Converte amostras de uma taxa/quantidade de canais para outra (interpolação linear).
pub(crate) struct Converter {
    pub(crate) in_rate: u32,
    pub(crate) in_channels: usize,
    out_rate: u32,
    out_channels: usize,
    /// Posição (em quadros de entrada) do próximo quadro de saída; 0 = o quadro anterior.
    t: f64,
    previous: Vec<f32>,
    /// Correção da taxa (1 = nenhuma): acima de 1 consome a entrada mais depressa.
    pub(crate) ratio: f64,
}

impl Converter {
    pub(crate) fn new(in_rate: u32, in_channels: u16, out_rate: u32, out_channels: u16) -> Self {
        Self {
            in_rate,
            in_channels: in_channels.max(1) as usize,
            out_rate,
            out_channels: out_channels.max(1) as usize,
            t: 1.0,
            previous: vec![0.0; in_channels.max(1) as usize],
            ratio: 1.0,
        }
    }

    /// Recomeça (depois de uma busca).
    pub(crate) fn reset(&mut self) {
        self.t = 1.0;
        self.previous.iter_mut().for_each(|s| *s = 0.0);
    }

    fn map_channels(&self, frame: &[f32], out: &mut VecDeque<f32>, volume: f32) {
        let (inc, outc) = (self.in_channels, self.out_channels);
        for c in 0..outc {
            let s = if inc == 1 {
                frame[0]
            } else if outc == 1 {
                (frame[0] + frame[1]) * 0.5
            } else if c < inc {
                frame[c]
            } else {
                0.0
            };
            out.push_back(s * volume);
        }
    }

    pub(crate) fn process(&mut self, input: &[f32], out: &mut VecDeque<f32>, volume: f32) {
        let ch = self.in_channels;
        let frames = input.len() / ch;
        if frames == 0 {
            return;
        }
        let at = |i: usize, c: usize, previous: &[f32]| if i == 0 { previous[c] } else { input[(i - 1) * ch + c] };
        let step = self.in_rate as f64 / self.out_rate as f64 * self.ratio;
        let mut frame = vec![0.0; ch];
        // índices: 0 = quadro anterior, 1..=frames = os novos
        while self.t <= frames as f64 {
            let a = self.t.floor() as usize;
            let b = (a + 1).min(frames);
            let f = (self.t - a as f64) as f32;
            for (c, v) in frame.iter_mut().enumerate() {
                let x = at(a, c, &self.previous);
                let y = at(b, c, &self.previous);
                *v = x + (y - x) * f;
            }
            self.map_channels(&frame, out, volume);
            self.t += step;
        }
        self.t -= frames as f64;
        self.previous.copy_from_slice(&input[(frames - 1) * ch..frames * ch]);
    }
}

// ---------------------------------------------------------------------------
// Saída

#[derive(Default)]
struct Control {
    playing: bool,
    volume: f32,
    looping: bool,
    seek: Option<f64>,
    quit: bool,
    finished: bool,
    /// Posição (segundos) na última busca; somada ao que já tocou desde então.
    base: f64,
    /// Saídas novas para a thread abrir.
    outputs: Option<Vec<OutputSpec>>,
}

struct Shared {
    control: Mutex<Control>,
    /// A fila da primeira saída: o relógio da posição.
    master: Mutex<Option<Arc<SinkQueue>>>,
    /// O silêncio posto na frente da primeira saída (o atraso dela), em quadros.
    master_delay: AtomicU64,
    taps: Taps,
}

/// Toca uma [`AudioSource`] numa ou em várias saídas de áudio ([`OutputSpec`]), numa
/// thread própria.
pub struct AudioPlayer {
    shared: Arc<Shared>,
    thread: Option<JoinHandle<()>>,
    duration: Option<f64>,
}

impl AudioPlayer {
    /// Começa parado, na saída padrão. `on_end` é chamado (na thread de áudio) quando a fonte
    /// termina sem `looping`.
    pub fn start(source: Box<dyn AudioSource + Send>, on_end: Option<Box<dyn Fn() + Send>>) -> Result<Self, String> {
        Self::open(Box::new(move || Ok(source as Box<dyn AudioSource>)), on_end)
    }

    /// Como [`start`](Self::start), abrindo a fonte já na thread do áudio (onde ela vai viver).
    pub fn open(opener: AudioOpener, on_end: Option<Box<dyn Fn() + Send>>) -> Result<Self, String> {
        Self::open_on(opener, on_end, vec![OutputSpec::new("")])
    }

    /// Como [`open`](Self::open), nas saídas `outputs` (o mesmo som sincronizado em todas).
    /// Uma saída que não abre é avisada no console; nenhuma abrindo, é erro.
    pub fn open_on(opener: AudioOpener, on_end: Option<Box<dyn Fn() + Send>>, outputs: Vec<OutputSpec>) -> Result<Self, String> {
        let shared = Arc::new(Shared {
            control: Mutex::new(Control { volume: 1.0, ..Default::default() }),
            master: Mutex::new(None),
            master_delay: AtomicU64::new(0),
            taps: Taps::default(),
        });
        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Result<Option<f64>, String>>();
        let thread_shared = shared.clone();
        let thread = std::thread::Builder::new()
            .name("aylis-audio".into())
            .spawn(move || audio_thread(thread_shared, opener, on_end, outputs, ready_tx))
            .map_err(|e| e.to_string())?;
        // A thread avisa se conseguiu abrir a fonte e a saída de áudio (e a duração).
        match ready_rx.recv_timeout(Duration::from_secs(5)) {
            Ok(Ok(duration)) => Ok(Self { shared, thread: Some(thread), duration }),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("a saída de áudio não respondeu".into()),
        }
    }

    fn control(&self, f: impl FnOnce(&mut Control)) {
        if let Ok(mut c) = self.shared.control.lock() {
            f(&mut c);
        }
    }

    pub fn play(&self) {
        self.control(|c| {
            if c.finished {
                c.finished = false;
                c.seek = Some(0.0);
            }
            c.playing = true;
        });
    }

    pub fn pause(&self) {
        self.control(|c| c.playing = false);
    }

    pub fn stop(&self) {
        self.control(|c| {
            c.playing = false;
            c.seek = Some(0.0);
        });
    }

    pub fn seek(&self, seconds: f64) {
        self.control(|c| {
            c.seek = Some(seconds.max(0.0));
            c.finished = false;
        });
    }

    pub fn set_volume(&self, volume: f32) {
        self.control(|c| c.volume = volume.clamp(0.0, 2.0));
    }

    pub fn set_looping(&self, looping: bool) {
        self.control(|c| c.looping = looping);
    }

    /// Troca as saídas (continua do ponto em que estava).
    pub fn set_outputs(&self, outputs: Vec<OutputSpec>) {
        self.control(|c| c.outputs = Some(outputs));
    }

    /// Um ponto de escuta com o que o player toca (já com o volume dele), para o
    /// [`AudioMixer`](crate::AudioMixer).
    pub fn tap(&self) -> Tap {
        self.shared.taps.add(MIX_RATE, MIX_CHANNELS)
    }

    pub fn is_playing(&self) -> bool {
        self.shared.control.lock().is_ok_and(|c| c.playing && !c.finished)
    }

    /// Posição em segundos (o que já saiu no alto-falante).
    pub fn position(&self) -> f64 {
        let base = self.shared.control.lock().map_or(0.0, |c| c.seek.unwrap_or(c.base));
        base + played_seconds(&self.shared)
    }

    pub fn duration(&self) -> Option<f64> {
        self.duration
    }
}

/// O que já tocou na primeira saída desde a última busca (sem o atraso dela).
fn played_seconds(shared: &Shared) -> f64 {
    let Some(master) = shared.master.lock().ok().and_then(|m| m.clone()) else { return 0.0 };
    let played = master.played.load(Ordering::Relaxed).saturating_sub(shared.master_delay.load(Ordering::Relaxed));
    played as f64 / master.rate.max(1) as f64
}

impl Drop for AudioPlayer {
    /// Avisa a thread e não espera: quem libera é a thread da interface, e uma fonte presa
    /// (rede lenta) congelaria a tela. A thread sai sozinha e solta a fonte e o dispositivo.
    fn drop(&mut self) {
        self.control(|c| c.quit = true);
        drop(self.thread.take());
    }
}

/// Uma saída aberta pelo player, com o conversor e o volume e o atraso dela.
struct Out {
    sink: Sink,
    converter: Converter,
    spec: OutputSpec,
}

impl Out {
    fn delay_frames(&self) -> usize {
        (self.sink.queue.rate as f32 * self.spec.delay_ms / 1000.0) as usize
    }
}

/// Abre as saídas; as que falham vão para o console. Nenhuma: o erro da primeira.
fn open_outputs(specs: &[OutputSpec], rate: u32, channels: u16) -> Result<Vec<Out>, String> {
    let mut outs = Vec::new();
    let mut first_error = None;
    for spec in specs {
        match Sink::open(&spec.name) {
            Ok(sink) => {
                let converter = Converter::new(rate, channels, sink.queue.rate, sink.queue.channels);
                outs.push(Out { sink, converter, spec: spec.clone() });
            }
            Err(e) => {
                aylis_core::stderr_line!("aylis-media: {e}");
                first_error.get_or_insert(e);
            }
        }
    }
    if outs.is_empty() {
        return Err(first_error.unwrap_or_else(|| aylis_core::messages::t("nenhuma saída de áudio")));
    }
    Ok(outs)
}

fn audio_thread(
    shared: Arc<Shared>,
    opener: AudioOpener,
    on_end: Option<Box<dyn Fn() + Send>>,
    specs: Vec<OutputSpec>,
    ready: std::sync::mpsc::Sender<Result<Option<f64>, String>>,
) {
    let mut source = match opener() {
        Ok(s) => s,
        Err(e) => {
            let _ = ready.send(Err(e));
            return;
        }
    };
    let mut outs = match open_outputs(&specs, source.sample_rate(), source.channels()) {
        Ok(o) => o,
        Err(e) => {
            let _ = ready.send(Err(e));
            return;
        }
    };
    let restart = |outs: &mut [Out]| {
        for o in outs.iter_mut() {
            o.converter.reset();
            o.sink.queue.reset(o.spec.delay_ms);
        }
        if let Ok(mut m) = shared.master.lock() {
            *m = Some(outs[0].sink.queue.clone());
        }
        shared.master_delay.store(outs[0].delay_frames() as u64, Ordering::Relaxed);
    };
    restart(&mut outs);
    let _ = ready.send(Ok(source.duration()));

    let mut buffer = Vec::new();
    let mut ended = false;
    loop {
        let (quit, mut seek, playing, volume, looping, new_outputs) = {
            let Ok(mut c) = shared.control.lock() else { break };
            (c.quit, c.seek.take(), c.playing, c.volume, c.looping, c.outputs.take())
        };
        if quit {
            break;
        }
        if let Some(specs) = new_outputs {
            match open_outputs(&specs, source.sample_rate(), source.channels()) {
                Ok(o) => {
                    // Volta a fonte ao que estava saindo no alto-falante e recomeça nas novas.
                    let at = shared.control.lock().map_or(0.0, |c| c.base) + played_seconds(&shared);
                    outs = o;
                    seek = Some(seek.unwrap_or(at));
                }
                Err(e) => aylis_core::stderr_line!("aylis-media: {e}"),
            }
        }
        for o in &outs {
            o.sink.queue.playing.store(playing, Ordering::Relaxed);
        }
        if let Some(t) = seek {
            source.seek(t);
            restart(&mut outs);
            if let Ok(mut c) = shared.control.lock() {
                c.base = t;
                c.finished = false;
            }
            ended = false;
        }
        let master = outs[0].sink.queue.clone();
        let queued = master.queued_frames();
        // ~250 ms na fila da primeira saída (fora o atraso dela).
        let target = master.rate as usize / 4 + outs[0].delay_frames();
        if playing && !ended && queued < target {
            buffer.clear();
            if source.read(&mut buffer) {
                let (rate, channels) = (source.sample_rate(), source.channels());
                // As outras saídas seguem a primeira: a fila de cada uma deve ter o mesmo tempo
                // (mais a diferença de atraso); a taxa corrige devagar o relógio de cada placa.
                let master_time = queued as f64 / master.rate.max(1) as f64 - outs[0].spec.delay_ms as f64 / 1000.0;
                for (i, o) in outs.iter_mut().enumerate() {
                    if o.converter.in_rate != rate || o.converter.in_channels != channels.max(1) as usize {
                        o.converter = Converter::new(rate, channels, o.sink.queue.rate, o.sink.queue.channels);
                    }
                    if i > 0 {
                        let out_rate = o.sink.queue.rate as f64;
                        let expected = (master_time + o.spec.delay_ms as f64 / 1000.0) * out_rate;
                        o.converter.ratio = drift_correction(o.sink.queue.queued_frames() as f64, expected, out_rate);
                    }
                    if let Ok(mut q) = o.sink.queue.queue.lock() {
                        o.converter.process(&buffer, &mut q, volume * o.spec.volume);
                    }
                }
                if !shared.taps.is_empty() {
                    let scaled: Vec<f32> = buffer.iter().map(|s| s * volume).collect();
                    shared.taps.feed(&scaled, rate, channels);
                }
                continue;
            } else if looping {
                if let Ok(mut c) = shared.control.lock() {
                    c.seek = Some(0.0);
                }
                continue;
            } else {
                ended = true;
            }
        }
        // Acabou e a fila esvaziou: avisa uma vez.
        if ended && queued == 0 {
            let notify = shared.control.lock().is_ok_and(|mut c| {
                let first = !c.finished;
                c.finished = true;
                c.playing = false;
                first
            });
            if notify {
                if let Some(f) = &on_end {
                    f();
                }
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_rate_and_channels() {
        // 1 s de mono a 8 kHz → estéreo a 16 kHz: o dobro de quadros, dois canais.
        let input: Vec<f32> = (0..8000).map(|i| i as f32 / 8000.0).collect();
        let mut c = Converter::new(8000, 1, 16000, 2);
        let mut out = VecDeque::new();
        for chunk in input.chunks(1000) {
            c.process(chunk, &mut out, 1.0);
        }
        let frames = out.len() / 2;
        assert!((15990..=16010).contains(&frames), "{frames} quadros");
        let v: Vec<f32> = out.into_iter().collect();
        assert_eq!(v[2000], v[2001], "os dois canais iguais");
        // quadro de saída 1000 = quadro de entrada 500 (taxa dobrada)
        assert!((v[2000] - 500.0 / 8000.0).abs() < 0.001, "interpolado: {}", v[2000]);
    }
}
