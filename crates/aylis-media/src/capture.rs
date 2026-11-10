//! Microfone: a entrada de áudio do sistema pelo `cpal` (WASAPI no Windows, CoreAudio,
//! ALSA/PulseAudio, AAudio/OpenSL no Android). Uma thread mantém o fluxo aberto; o nível
//! (para um medidor) e a gravação em WAV ficam na memória compartilhada.

use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use crate::audio::Converter;
use crate::output::{OutputSpec, Sink, SinkQueue, Tap, Taps, drift_correction};

/// O começo do nome de uma entrada que é o som de uma saída (loopback):
/// `loopback:Alto-falantes`.
pub const LOOPBACK_PREFIX: &str = "loopback:";

/// O sistema grava o som das saídas: WASAPI no Windows e o agregado do CoreAudio no macOS
/// 14.4+. No Linux, o monitor de cada saída do PipeWire/Pulse já aparece como microfone.
pub const HAS_LOOPBACK: bool = cfg!(any(windows, target_os = "macos"));

/// O loopback do Android: o som que os outros apps tocam (o `AudioPlaybackCapture`, com a
/// autorização do usuário na tela do sistema).
pub const ANDROID_LOOPBACK: &str = "loopback:Android";

/// As entradas do sistema (o nome de cada uma): os microfones e, onde o sistema deixa, o
/// som de cada saída (`loopback:<saída>`).
pub fn input_devices() -> Vec<String> {
    use cpal::traits::{DeviceTrait, HostTrait};
    let mut list: Vec<String> =
        cpal::default_host().input_devices().map(|ds| ds.filter_map(|d| d.description().ok().map(|d| d.name().to_string())).collect()).unwrap_or_default();
    if HAS_LOOPBACK {
        list.extend(crate::output::output_devices().into_iter().map(|o| format!("{LOOPBACK_PREFIX}{}", o.name)));
    }
    if cfg!(target_os = "android") {
        list.push(ANDROID_LOOPBACK.to_string());
    }
    list
}

/// A saída mandada de volta para alto-falantes ou fones (`monitor`), com o conversor dela.
struct Monitor {
    queue: Arc<SinkQueue>,
    converter: Converter,
    spec: OutputSpec,
}

#[derive(Default)]
struct Recording {
    samples: Vec<i16>,
}

struct Shared {
    /// O pico recente (0 a 1), em bits de `f32`.
    level: AtomicU32,
    quit: AtomicBool,
    recording: Mutex<Option<Recording>>,
    rate: AtomicU32,
    channels: AtomicU32,
    monitors: Mutex<Vec<Monitor>>,
    /// Saídas novas do `monitor` para a thread abrir (vazio: nenhuma).
    pending_monitor: Mutex<Option<Vec<OutputSpec>>>,
    taps: Taps,
}

/// O microfone aberto. Soltar fecha.
pub struct AudioInput {
    shared: Arc<Shared>,
}

impl AudioInput {
    /// Abre o microfone `device` (vazio: o padrão do sistema).
    pub fn open(device: &str) -> Result<Self, String> {
        let shared = Arc::new(Shared {
            level: AtomicU32::new(0),
            quit: AtomicBool::new(false),
            recording: Mutex::new(None),
            rate: AtomicU32::new(0),
            channels: AtomicU32::new(0),
            monitors: Mutex::new(Vec::new()),
            pending_monitor: Mutex::new(None),
            taps: Taps::default(),
        });
        let (ready, wait) = std::sync::mpsc::channel();
        let thread_shared = shared.clone();
        let device = device.to_string();
        std::thread::Builder::new().name("aylis-mic".into()).spawn(move || input_thread(thread_shared, &device, ready)).map_err(|e| e.to_string())?;
        wait.recv().map_err(|_| "microfone: a thread parou".to_string())??;
        Ok(Self { shared })
    }

    /// O nível do som agora (0 = silêncio, 1 = o máximo), com queda suave.
    pub fn level(&self) -> f32 {
        f32::from_bits(self.shared.level.load(Ordering::Relaxed))
    }

    /// Taxa de amostragem e canais do microfone.
    pub fn format(&self) -> (u32, u16) {
        (self.shared.rate.load(Ordering::Relaxed), self.shared.channels.load(Ordering::Relaxed) as u16)
    }

    /// Manda a entrada para estas saídas, cada uma com o volume e o atraso dela (vazio:
    /// para de mandar). Cuidado com o microfone perto do alto-falante (microfonia).
    pub fn set_monitor(&self, outputs: Vec<OutputSpec>) {
        if let Ok(mut p) = self.shared.pending_monitor.lock() {
            *p = Some(outputs);
        }
    }

    /// Um ponto de escuta com o que entra, para o [`AudioMixer`](crate::AudioMixer).
    pub fn tap(&self) -> Tap {
        let (rate, channels) = self.format();
        self.shared.taps.add(rate.max(1), channels.max(1))
    }

    pub fn is_recording(&self) -> bool {
        self.shared.recording.lock().is_ok_and(|r| r.is_some())
    }

    /// Começa a gravar (o que vier antes é descartado).
    pub fn start_recording(&self) {
        if let Ok(mut r) = self.shared.recording.lock() {
            *r = Some(Recording::default());
        }
    }

    /// Para de gravar e salva em WAV (PCM 16 bits). Devolve a duração em segundos.
    pub fn stop_recording(&self, path: &Path) -> Result<f64, String> {
        let rec = self.shared.recording.lock().map_err(|e| e.to_string())?.take().ok_or("não estava gravando")?;
        let (rate, channels) = self.format();
        write_wav(path, &rec.samples, rate, channels)?;
        Ok(rec.samples.len() as f64 / (rate.max(1) as f64 * channels.max(1) as f64))
    }
}

impl Drop for AudioInput {
    fn drop(&mut self) {
        self.shared.quit.store(true, Ordering::Relaxed);
    }
}

/// Grava amostras PCM de 16 bits num WAV.
pub fn write_wav(path: &Path, samples: &[i16], rate: u32, channels: u16) -> Result<(), String> {
    let data_len = (samples.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&channels.to_le_bytes());
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * channels as u32 * 2).to_le_bytes());
    out.extend_from_slice(&(channels * 2).to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

fn input_thread(shared: Arc<Shared>, device: &str, ready: std::sync::mpsc::Sender<Result<(), String>>) {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    #[cfg(target_os = "android")]
    if device == ANDROID_LOOPBACK {
        return android_loopback(&shared, &ready);
    }
    let host = cpal::default_host();
    // Loopback: a entrada é aberta na própria saída (o sistema entrega o que ela toca).
    let loopback = device.strip_prefix(LOOPBACK_PREFIX).filter(|_| HAS_LOOPBACK);
    let found = match loopback {
        Some(output) => crate::output::find_output(output),
        None if device.is_empty() => host.default_input_device(),
        None => host.input_devices().ok().and_then(|mut ds| ds.find(|d| d.description().is_ok_and(|n| n.name() == device))),
    };
    let Some(device) = found else {
        let _ = ready.send(Err(aylis_core::messages::t("nenhum microfone")));
        return;
    };
    let config = if loopback.is_some() { device.default_output_config() } else { device.default_input_config() };
    let config = match config {
        Ok(c) => c,
        Err(e) => {
            let _ = ready.send(Err(format!("microfone: {e}")));
            return;
        }
    };
    shared.rate.store(config.sample_rate(), Ordering::Relaxed);
    shared.channels.store(config.channels() as u32, Ordering::Relaxed);
    let stream_config: cpal::StreamConfig = config.clone().into();
    let (rate, channels) = (config.sample_rate(), config.channels());
    let mut take = sample_sink(shared.clone(), rate, channels);
    let error = |e| aylis_core::stderr_line!("aylis-media: erro no microfone: {e}");
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(stream_config, move |d: &[f32], _| take(&mut d.iter().copied()), error, None),
        cpal::SampleFormat::I16 => {
            device.build_input_stream(stream_config, move |d: &[i16], _| take(&mut d.iter().map(|&s| s as f32 / i16::MAX as f32)), error, None)
        }
        cpal::SampleFormat::U16 => device.build_input_stream(
            stream_config,
            move |d: &[u16], _| take(&mut d.iter().map(|&s| s as f32 / u16::MAX as f32 * 2.0 - 1.0)),
            error,
            None,
        ),
        other => {
            let _ = ready.send(Err(format!("formato do microfone {other:?} não suportado")));
            return;
        }
    };
    let stream = match stream {
        Ok(s) => s,
        Err(e) => {
            let _ = ready.send(Err(format!("microfone: {e}")));
            return;
        }
    };
    if let Err(e) = stream.play() {
        let _ = ready.send(Err(format!("microfone: {e}")));
        return;
    }
    let _ = ready.send(Ok(()));
    // As saídas do monitor vivem nesta thread (o fluxo do sistema não muda de thread).
    let mut sinks: Vec<Sink> = Vec::new();
    while !shared.quit.load(Ordering::Relaxed) {
        let pending = shared.pending_monitor.lock().ok().and_then(|mut p| p.take());
        if let Some(specs) = pending {
            if let Ok(mut m) = shared.monitors.lock() {
                m.clear();
            }
            sinks.clear();
            let mut monitors = Vec::new();
            for spec in specs {
                match Sink::open(&spec.name) {
                    Ok(sink) => {
                        sink.queue.reset(spec.delay_ms);
                        sink.queue.playing.store(true, Ordering::Relaxed);
                        let converter = Converter::new(rate, channels, sink.queue.rate, sink.queue.channels);
                        monitors.push(Monitor { queue: sink.queue.clone(), converter, spec });
                        sinks.push(sink);
                    }
                    Err(e) => aylis_core::stderr_line!("aylis-media: monitor: {e}"),
                }
            }
            if let Ok(mut m) = shared.monitors.lock() {
                *m = monitors;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

/// Manda o bloco que entrou para as saídas do monitor. A fila de cada uma fica perto de
/// 40 ms (mais o atraso dela): o relógio da entrada e o da saída andam diferente, e a taxa
/// corrige devagar; passando muito, joga fora o excesso.
fn feed_monitors(shared: &Shared, block: &[f32]) {
    let Ok(mut monitors) = shared.monitors.lock() else { return };
    for m in monitors.iter_mut() {
        let out_rate = m.queue.rate as f64;
        let target = (0.04 + m.spec.delay_ms as f64 / 1000.0) * out_rate;
        let queued = m.queue.queued_frames() as f64;
        m.converter.ratio = drift_correction(queued, target, out_rate);
        if let Ok(mut q) = m.queue.queue.lock() {
            if queued > target + out_rate * 0.2 {
                let keep = target as usize * m.queue.channels as usize;
                let extra = q.len().saturating_sub(keep);
                q.drain(..extra);
            }
            m.converter.process(block, &mut q, m.spec.volume);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn wav_header() {
        let path = std::env::temp_dir().join(format!("aylis_wav_{}.wav", std::process::id()));
        super::write_wav(&path, &[0, 1000, -1000, 0], 48_000, 2).unwrap();
        let b = std::fs::read(&path).unwrap();
        assert_eq!(&b[0..4], b"RIFF");
        assert_eq!(u32::from_le_bytes(b[24..28].try_into().unwrap()), 48_000);
        assert_eq!(u32::from_le_bytes(b[40..44].try_into().unwrap()), 8);
        assert_eq!(b.len(), 44 + 8);
        let _ = std::fs::remove_file(path);
    }
}

/// O que fazer com cada bloco de amostras que chega (de qualquer fonte): gravar, medir o
/// nível, mandar para o monitor e para quem escuta (o mixer).
fn sample_sink(shared: Arc<Shared>, rate: u32, channels: u16) -> impl FnMut(&mut dyn Iterator<Item = f32>) {
    let mut block = Vec::new();
    move |samples: &mut dyn Iterator<Item = f32>| {
        let mut peak = 0f32;
        block.clear();
        block.extend(samples);
        if let Ok(mut rec) = shared.recording.lock() {
            if let Some(r) = rec.as_mut() {
                r.samples.extend(block.iter().map(|s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16));
            }
        }
        for s in &block {
            peak = peak.max(s.abs());
        }
        feed_monitors(&shared, &block);
        if !shared.taps.is_empty() {
            shared.taps.feed(&block, rate, channels);
        }
        // Sobe na hora, desce devagar (como um VU).
        let old = f32::from_bits(shared.level.load(Ordering::Relaxed));
        let level = if peak > old { peak } else { old * 0.85 + peak * 0.15 };
        shared.level.store(level.min(1.0).to_bits(), Ordering::Relaxed);
    }
}

/// O loopback do Android: a `AylisActivity` pede a autorização (a tela do sistema), sobe o
/// serviço de captura e grava o som dos outros apps num `AudioRecord`; esta thread lê as
/// amostras (48 kHz, estéreo, 16 bits) pelo JNI até o microfone ser fechado.
#[cfg(target_os = "android")]
fn android_loopback(shared: &Arc<Shared>, ready: &std::sync::mpsc::Sender<Result<(), String>>) {
    let ctx = ndk_context::android_context();
    // SAFETY: a VM do processo, mantida pelo android-activity.
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else {
        let _ = ready.send(Err("loopback: sem a VM".into()));
        return;
    };
    let Ok(mut env) = vm.attach_current_thread() else {
        let _ = ready.send(Err("loopback: sem a thread".into()));
        return;
    };
    let Some(activity) = crate::android_camera::aylis_activity(&mut env) else {
        let _ = ready.send(Err("loopback: sem a AylisActivity".into()));
        return;
    };
    let (rate, channels) = (48_000u32, 2u16);
    shared.rate.store(rate, Ordering::Relaxed);
    shared.channels.store(u32::from(channels), Ordering::Relaxed);
    let mut take = sample_sink(shared.clone(), rate, channels);
    let Ok(buffer) = env.new_short_array(4096) else {
        let _ = ready.send(Err("loopback: memória".into()));
        return;
    };
    let mut samples = vec![0i16; 4096];
    // Pronta já: o som chega quando o usuário autorizar (esperar aqui travaria quem abriu).
    let _ = ready.send(Ok(()));
    let began = std::time::Instant::now();
    while !shared.quit.load(Ordering::Relaxed) {
        let state = env.call_method(&activity, "loopbackStart", "()I", &[]).and_then(|v| v.i()).unwrap_or(-1);
        match state {
            // Gravando: lê o que houver.
            2 => {
                let n = env.call_method(&activity, "loopbackRead", "([S)I", &[(&buffer).into()]).and_then(|v| v.i()).unwrap_or(0);
                if n > 0 && env.get_short_array_region(&buffer, 0, &mut samples[..n as usize]).is_ok() {
                    take(&mut samples[..n as usize].iter().map(|&s| f32::from(s) / f32::from(i16::MAX)));
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
            // Esperando o usuário autorizar (até 2 minutos).
            1 if began.elapsed().as_secs() < 120 => std::thread::sleep(std::time::Duration::from_millis(200)),
            _ => {
                let _ = env.exception_clear();
                crate::android_camera::log("loopback: o som dos apps não foi autorizado");
                break;
            }
        }
    }
    let _ = env.call_method(&activity, "loopbackStop", "()V", &[]);
}
