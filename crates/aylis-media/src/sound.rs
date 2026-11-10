//! Componentes de áudio não visuais: [`AudioOutput`] (as saídas do sistema), [`Sound`]
//! (efeitos e alertas) e [`AudioMixer`] (mistura microfones, loopback e players).

use std::cell::RefCell;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use aylis_core::{Component, ComponentId, EventArgs, EventCtx, MethodInfo, PropKind, PropValue, Ui, properties};

use crate::audio::{AudioPlayer, AudioSource, Converter, SymphoniaSource};
use crate::capture::{AudioInput, write_wav};
use crate::output::{MIX_CHANNELS, MIX_RATE, OutputDevice, OutputSpec, Sink, SinkQueue, Tap, drift_correction, output_devices, parse_outputs};

// ---------------------------------------------------------------------------

/// As saídas de áudio do sistema (como uma lista viva). Não visual.
///
/// - `devices`: os nomes, separados por `;` (para o `items` de um ComboBox);
///   `default_device`: a saída padrão.
/// - `on_devices_changed`: um fone ou uma caixa Bluetooth entrou ou saiu (conferido a cada
///   2 s com `watch` ligado).
///
/// Para tocar numa delas, ponha o nome no `output` do [`MediaPlayer`](crate::MediaPlayer),
/// do [`VideoPlayer`](crate::VideoPlayer) ou do [`Sound`].
pub struct AudioOutput {
    pub watch: bool,
    list: RefCell<Option<Vec<OutputDevice>>>,
    busy: Arc<AtomicBool>,
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self { watch: true, list: RefCell::default(), busy: Arc::default() }
    }
}

impl AudioOutput {
    /// As saídas (com canais, taxa e qual é a padrão).
    pub fn devices(ui: &Ui, id: ComponentId) -> Vec<OutputDevice> {
        ui.get::<AudioOutput>(id).map(AudioOutput::current).unwrap_or_default()
    }

    fn current(&self) -> Vec<OutputDevice> {
        self.list.borrow_mut().get_or_insert_with(output_devices).clone()
    }

    fn device_names(&self) -> String {
        self.current().iter().map(|d| d.name.clone()).collect::<Vec<_>>().join(";")
    }

    fn set_device_names(&mut self, _: String) {}

    fn default_device(&self) -> String {
        self.current().into_iter().find(|d| d.is_default).map(|d| d.name).unwrap_or_default()
    }

    fn set_default_device(&mut self, _: String) {}
}

impl Component for AudioOutput {
    fn class_name(&self) -> &'static str {
        "AudioOutput"
    }

    properties! {
        watch: bool => "Behavior";
        events: on_devices_changed;
        computed: devices: String => device_names / set_device_names, default_device: String => default_device / set_default_device
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "watch" {
            ctx.ui.sync_timer(ctx.id);
        }
    }

    fn timer_interval(&self) -> Option<f64> {
        self.watch.then_some(2.0)
    }

    /// Confere a lista numa thread (o sistema pode demorar a responder) e avisa se mudou.
    fn timer(&mut self, ctx: &mut EventCtx<'_>) {
        if ctx.ui.design_mode() || self.busy.swap(true, Ordering::AcqRel) {
            return;
        }
        let (poster, id, busy) = (ctx.ui.poster(), ctx.id, self.busy.clone());
        std::thread::spawn(move || {
            let fresh = output_devices();
            busy.store(false, Ordering::Release);
            poster.post(move |ui| {
                let changed = ui.get::<AudioOutput>(id).is_some_and(|o| {
                    let mut list = o.list.borrow_mut();
                    let changed = list.as_ref().is_some_and(|old| *old != fresh);
                    *list = Some(fresh);
                    changed
                });
                if changed {
                    ui.fire(id, "on_devices_changed", &EventArgs::None);
                }
            });
        });
    }
}

// ---------------------------------------------------------------------------

/// Um som curto (efeito, alerta, clique): `Sound::play` toca do começo, e chamar de novo
/// antes de acabar toca por cima (até `polyphony` ao mesmo tempo). O arquivo fica na
/// memória depois da primeira vez. Não visual.
pub struct Sound {
    /// Arquivo do aplicativo (`assets/audio/clique.wav`) ou caminho no disco.
    pub file: String,
    pub volume: f32,
    /// As saídas (como no [`MediaPlayer`](crate::MediaPlayer)).
    pub output: String,
    /// Quantos toques ao mesmo tempo (o mais antigo para quando passa).
    pub polyphony: i64,
    bytes: Option<(String, Arc<Vec<u8>>)>,
    voices: Vec<AudioPlayer>,
}

impl Default for Sound {
    fn default() -> Self {
        Self { file: String::new(), volume: 1.0, output: String::new(), polyphony: 4, bytes: None, voices: Vec::new() }
    }
}

impl Sound {
    /// Toca (por cima do que já está tocando). O erro também vai para o `on_error`.
    pub fn play(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        let result = Self::start(ui, id);
        if let Err(e) = &result {
            ui.fire(id, "on_error", &EventArgs::Text(e.clone()));
        }
        result
    }

    fn start(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        let (file, cached) = match ui.get::<Sound>(id) {
            Some(s) => (s.file.clone(), s.bytes.as_ref().filter(|(f, _)| *f == s.file).map(|(_, b)| b.clone())),
            None => return Err("não é um Sound".into()),
        };
        if file.is_empty() {
            return Err("informe o arquivo (file)".into());
        }
        let bytes = match cached {
            Some(b) => b,
            None => {
                let read = ui.services().resources.as_ref().and_then(|r| r.read(&file));
                let bytes = match read {
                    Some(b) => b,
                    None => std::fs::read(&file).map_err(|e| format!("{file}: {e}"))?,
                };
                Arc::new(bytes)
            }
        };
        let extension = Path::new(&file).extension().and_then(|e| e.to_str()).unwrap_or_default().to_lowercase();
        let (volume, outputs) = ui.get::<Sound>(id).map(|s| (s.volume, parse_outputs(&s.output))).unwrap_or((1.0, Vec::new()));
        let data = bytes.clone();
        let opener = Box::new(move || SymphoniaSource::open(data.to_vec(), Some(&extension)).map(|s| Box::new(s) as Box<dyn AudioSource>));
        let voice = AudioPlayer::open_on(opener, None, outputs)?;
        voice.set_volume(volume);
        voice.play();
        if let Some(s) = ui.get_mut::<Sound>(id) {
            s.bytes = Some((file, bytes));
            s.voices.retain(AudioPlayer::is_playing);
            let max = s.polyphony.max(1) as usize;
            while s.voices.len() >= max {
                s.voices.remove(0);
            }
            s.voices.push(voice);
        }
        Ok(())
    }

    /// Para todos os toques.
    pub fn stop(ui: &mut Ui, id: ComponentId) {
        if let Some(s) = ui.get_mut::<Sound>(id) {
            s.voices.clear();
        }
    }

    pub fn is_playing(ui: &Ui, id: ComponentId) -> bool {
        ui.get::<Sound>(id).is_some_and(|s| s.voices.iter().any(AudioPlayer::is_playing))
    }

    fn sync(&mut self, name: &str) {
        match name {
            "volume" => self.voices.iter().for_each(|v| v.set_volume(self.volume)),
            "file" => self.voices.clear(),
            _ => {}
        }
    }
}

impl Component for Sound {
    fn class_name(&self) -> &'static str {
        "Sound"
    }

    properties! {
        file: String => "Behavior",
        volume: f32 => "Behavior",
        output: String => "Behavior",
        polyphony: i64 => "Behavior";
        events: on_error;
        on_set: sync
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] =
            &[MethodInfo::new("play", &[], Some(PropKind::Bool)), MethodInfo::new("stop", &[], None), MethodInfo::new("is_playing", &[], Some(PropKind::Bool))];
        METHODS
    }

    /// As outras linguagens (pelo protocolo): `play` sai pela fila, depois deste evento.
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _: &[PropValue]) -> Result<PropValue, String> {
        match method {
            "play" => {
                let id = ctx.id;
                ctx.ui.poster().post(move |ui| {
                    let _ = Sound::play(ui, id);
                });
                Ok(PropValue::Bool(true))
            }
            "stop" => {
                self.voices.clear();
                Ok(PropValue::Null)
            }
            "is_playing" => Ok(PropValue::Bool(self.voices.iter().any(AudioPlayer::is_playing))),
            _ => Err(format!("Sound não tem o método '{method}'")),
        }
    }
}

// ---------------------------------------------------------------------------

/// Uma fonte do mixer: `device:<entrada>` (um microfone ou `device:loopback:<saída>`) ou
/// o nome de um componente do form (um `Microphone`, `MediaPlayer` ou `VideoPlayer`).
#[derive(Clone, Debug, PartialEq)]
pub struct MixSource {
    pub name: String,
    /// `name` é uma entrada do sistema (não um componente).
    pub device: bool,
    pub volume: f32,
    /// -1 (esquerda) a 1 (direita).
    pub pan: f32,
    pub mute: bool,
}

/// O começo de uma fonte que é uma entrada do sistema.
pub const DEVICE_PREFIX: &str = "device:";

/// Lê a propriedade `sources` do mixer: fontes separadas por `|`, cada uma seguida, se
/// quiser, de `;volume=0.5;pan=-0.3;mute`.
///
/// `micVoz;volume=1.2|device:loopback:Alto-falantes;volume=0.4|mpFundo;pan=0.5;mute`
pub fn parse_sources(text: &str) -> Vec<MixSource> {
    text.split('|')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let mut parts = entry.split(';');
            let first = parts.next().unwrap_or_default().trim();
            let (device, name) = match first.strip_prefix(DEVICE_PREFIX) {
                Some(rest) => (true, rest.to_string()),
                None => (false, first.to_string()),
            };
            let mut source = MixSource { name, device, volume: 1.0, pan: 0.0, mute: false };
            for option in parts {
                let (key, value) = option.split_once('=').unwrap_or((option, ""));
                let number = value.trim().parse::<f32>().ok().filter(|v| v.is_finite());
                match (key.trim(), number) {
                    ("volume", Some(v)) => source.volume = v.clamp(0.0, 4.0),
                    ("pan", Some(v)) => source.pan = v.clamp(-1.0, 1.0),
                    ("mute", _) => source.mute = !matches!(value.trim(), "false" | "0"),
                    _ => {}
                }
            }
            source
        })
        .collect()
}

impl MixSource {
    /// O ganho dos canais esquerdo e direito (panorâmica linear: o centro fica inteiro nos
    /// dois lados).
    pub fn gains(&self) -> (f32, f32) {
        if self.mute {
            return (0.0, 0.0);
        }
        (self.volume * (1.0 - self.pan).min(1.0), self.volume * (1.0 + self.pan).min(1.0))
    }
}

/// O que o mixer guarda para quem chama: nível, gravação e amostras.
struct MixShared {
    quit: AtomicBool,
    level: AtomicU32,
    recording: Mutex<Option<Vec<i16>>>,
    /// As amostras para o código ([`AudioMixer::take_samples`]), até 5 s.
    samples: Mutex<Option<std::collections::VecDeque<f32>>>,
    gains: Mutex<Vec<(f32, f32)>>,
}

/// Mistura entradas (microfones e o som das saídas) e players, cada um com volume, mudo e
/// panorâmica, e entrega a mistura (48 kHz, estéreo) a saídas (`output`), a um arquivo WAV
/// (`AudioMixer::record` / `stop_recording`) ou ao código (`keep_samples` e
/// `AudioMixer::take_samples`). Não visual.
///
/// `sources`: veja [`parse_sources`]. `output` vazio: não toca em lugar nenhum (o mixer
/// costuma ir para arquivo ou para o código); a padrão é o nome dela ou `default`.
pub struct AudioMixer {
    pub sources: String,
    pub output: String,
    pub active: bool,
    pub keep_samples: bool,
    shared: Option<Arc<MixShared>>,
    /// As entradas que o mixer abriu ele mesmo (`device:`): vivem enquanto ele estiver ligado.
    inputs: Vec<AudioInput>,
    /// As fontes ligadas agora (nome e se é entrada do sistema).
    connected: Vec<(String, bool)>,
}

impl Default for AudioMixer {
    fn default() -> Self {
        Self { sources: String::new(), output: String::new(), active: false, keep_samples: false, shared: None, inputs: Vec::new(), connected: Vec::new() }
    }
}

impl AudioMixer {
    /// O nível da mistura agora (0 a 1).
    pub fn level(ui: &Ui, id: ComponentId) -> f32 {
        ui.get::<AudioMixer>(id).and_then(|m| m.shared.as_ref()).map_or(0.0, |s| f32::from_bits(s.level.load(Ordering::Relaxed)))
    }

    /// Começa a gravar a mistura (liga o mixer, se preciso).
    pub fn record(ui: &mut Ui, id: ComponentId) -> Result<(), String> {
        if !ui.get::<AudioMixer>(id).is_some_and(|m| m.active) {
            ui.set_prop(id, "active", true).map_err(|e| e.to_string())?;
        }
        let shared = ui.get::<AudioMixer>(id).and_then(|m| m.shared.clone()).ok_or("mixer desligado")?;
        *shared.recording.lock().map_err(|e| e.to_string())? = Some(Vec::new());
        Ok(())
    }

    /// Para de gravar e salva o WAV (48 kHz, estéreo, 16 bits); devolve a duração (s).
    pub fn stop_recording(ui: &Ui, id: ComponentId, path: &Path) -> Result<f64, String> {
        let shared = ui.get::<AudioMixer>(id).and_then(|m| m.shared.clone()).ok_or("mixer desligado")?;
        let samples = shared.recording.lock().map_err(|e| e.to_string())?.take().ok_or("não estava gravando")?;
        write_wav(path, &samples, MIX_RATE, MIX_CHANNELS)?;
        Ok(samples.len() as f64 / (MIX_RATE as f64 * MIX_CHANNELS as f64))
    }

    /// As amostras misturadas desde a última chamada (48 kHz, estéreo intercalado), com
    /// `keep_samples` ligado; guarda até 5 s.
    pub fn take_samples(ui: &Ui, id: ComponentId) -> Vec<f32> {
        let Some(shared) = ui.get::<AudioMixer>(id).and_then(|m| m.shared.clone()) else { return Vec::new() };
        shared.samples.lock().ok().and_then(|mut s| s.as_mut().map(|q| q.drain(..).collect())).unwrap_or_default()
    }

    fn stop(&mut self) {
        if let Some(s) = self.shared.take() {
            s.quit.store(true, Ordering::Relaxed);
        }
        self.inputs.clear();
    }

    /// Liga com as fontes e saídas atuais (ou desliga).
    fn restart(ui: &mut Ui, id: ComponentId) {
        let Some(mixer) = ui.get_mut::<AudioMixer>(id) else { return };
        mixer.stop();
        if !mixer.active || ui.design_mode() {
            return;
        }
        let (sources_text, output, keep) = match ui.get::<AudioMixer>(id) {
            Some(m) => (m.sources.clone(), m.output.clone(), m.keep_samples),
            None => return,
        };
        let sources = parse_sources(&sources_text);
        let mut taps = Vec::new();
        let mut inputs = Vec::new();
        let mut errors = Vec::new();
        for source in &sources {
            let tap = if source.device {
                match AudioInput::open(&source.name) {
                    Ok(input) => {
                        let tap = input.tap();
                        inputs.push(input);
                        Some(tap)
                    }
                    Err(e) => {
                        errors.push(format!("{}: {e}", source.name));
                        None
                    }
                }
            } else {
                let found = ui.find(&source.name);
                let tap = found.and_then(|c| match ui.class_name(c) {
                    Some("Microphone") => crate::Microphone::tap(ui, c),
                    Some("MediaPlayer") => crate::MediaPlayer::tap(ui, c),
                    Some("VideoPlayer") => crate::VideoPlayer::tap(ui, c),
                    _ => None,
                });
                if tap.is_none() {
                    errors.push(format!("{}: {}", source.name, aylis_core::messages::t("fonte não encontrada ou desligada")));
                }
                tap
            };
            // Uma fonte que falhou fica muda, mas no lugar (os ganhos seguem a ordem).
            taps.push(tap.unwrap_or_default());
        }
        let outputs = if output.trim().is_empty() {
            Vec::new()
        } else {
            parse_outputs(&output).into_iter().map(|mut s| {
                if s.name == "default" {
                    s.name.clear();
                }
                s
            }).collect()
        };
        let shared = Arc::new(MixShared {
            quit: AtomicBool::new(false),
            level: AtomicU32::new(0),
            recording: Mutex::new(None),
            samples: Mutex::new(keep.then(Default::default)),
            gains: Mutex::new(sources.iter().map(MixSource::gains).collect()),
        });
        let thread_shared = shared.clone();
        let spawned = std::thread::Builder::new().name("aylis-mixer".into()).spawn(move || mix_thread(thread_shared, taps, outputs));
        if let Err(e) = spawned {
            errors.push(e.to_string());
        }
        if let Some(m) = ui.get_mut::<AudioMixer>(id) {
            m.shared = Some(shared);
            m.inputs = inputs;
            m.connected = sources.iter().map(|s| (s.name.clone(), s.device)).collect();
        }
        for e in errors {
            ui.fire(id, "on_error", &EventArgs::Text(e));
        }
    }
}

impl Drop for AudioMixer {
    fn drop(&mut self) {
        self.stop();
    }
}

impl Component for AudioMixer {
    fn class_name(&self) -> &'static str {
        "AudioMixer"
    }

    properties! {
        sources: String => "Behavior",
        output: String => "Behavior",
        active: bool => "Behavior",
        keep_samples: bool => "Behavior";
        events: on_error
    }

    fn methods(&self) -> &'static [MethodInfo] {
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("record", &[], Some(PropKind::Bool)),
            MethodInfo::new("stop_recording", &[PropKind::String], Some(PropKind::Float)).params(&["path"]),
            MethodInfo::new("level", &[], Some(PropKind::Float)),
        ];
        METHODS
    }

    /// As outras linguagens (pelo protocolo). `record` precisa do mixer ligado (`active`).
    fn invoke(&mut self, _: &mut EventCtx<'_>, method: &str, args: &[PropValue]) -> Result<PropValue, String> {
        let shared = self.shared.clone().ok_or("mixer desligado")?;
        match method {
            "record" => {
                *shared.recording.lock().map_err(|e| e.to_string())? = Some(Vec::new());
                Ok(PropValue::Bool(true))
            }
            "stop_recording" => {
                let path = args.first().and_then(|v| v.to_str().ok()).ok_or("informe o arquivo")?;
                let samples = shared.recording.lock().map_err(|e| e.to_string())?.take().ok_or("não estava gravando")?;
                write_wav(Path::new(path), &samples, MIX_RATE, MIX_CHANNELS)?;
                Ok(PropValue::Float(samples.len() as f64 / (MIX_RATE as f64 * MIX_CHANNELS as f64)))
            }
            "level" => Ok(PropValue::Float(f32::from_bits(shared.level.load(Ordering::Relaxed)) as f64)),
            _ => Err(format!("AudioMixer não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        match name {
            // Volume, panorâmica e mudo mudam na hora, sem religar (as mesmas fontes).
            "sources" => {
                let parsed = parse_sources(&self.sources);
                let same = self.shared.is_some() && parsed.iter().map(|s| (s.name.clone(), s.device)).eq(self.connected.iter().cloned());
                if same {
                    if let Some(s) = &self.shared {
                        if let Ok(mut g) = s.gains.lock() {
                            *g = parsed.iter().map(MixSource::gains).collect();
                        }
                    }
                    return;
                }
                self.restart_later(ctx);
            }
            "output" | "active" | "keep_samples" => self.restart_later(ctx),
            _ => {}
        }
    }
}

impl AudioMixer {
    /// Religa fora do `property_changed` (que tem o componente emprestado).
    fn restart_later(&mut self, ctx: &mut EventCtx<'_>) {
        self.stop();
        let (poster, id) = (ctx.ui.poster(), ctx.id);
        poster.post(move |ui| AudioMixer::restart(ui, id));
    }
}

/// Uma saída do mixer, com o conversor (48 kHz estéreo → a placa) e o volume e atraso dela.
struct MixOut {
    _sink: Sink,
    queue: Arc<SinkQueue>,
    converter: Converter,
    spec: OutputSpec,
}

/// Soma as fontes a cada 10 ms (o tempo que passou, em quadros) e entrega a mistura.
fn mix_thread(shared: Arc<MixShared>, taps: Vec<Tap>, specs: Vec<OutputSpec>) {
    let mut outs: Vec<MixOut> = Vec::new();
    for spec in specs {
        match Sink::open(&spec.name) {
            Ok(sink) => {
                sink.queue.reset(spec.delay_ms);
                sink.queue.playing.store(true, Ordering::Relaxed);
                let converter = Converter::new(MIX_RATE, MIX_CHANNELS, sink.queue.rate, sink.queue.channels);
                outs.push(MixOut { queue: sink.queue.clone(), _sink: sink, converter, spec });
            }
            Err(e) => aylis_core::stderr_line!("aylis-media: mixer: {e}"),
        }
    }
    let mut last = Instant::now();
    let mut owed = 0.0f64;
    let mut mix = Vec::new();
    let mut block = Vec::new();
    while !shared.quit.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(10));
        let now = Instant::now();
        owed += now.duration_since(last).as_secs_f64() * MIX_RATE as f64;
        last = now;
        let frames = (owed as usize).min(MIX_RATE as usize / 5);
        owed = (owed - frames as f64).min(MIX_RATE as f64 / 5.0);
        if frames == 0 {
            continue;
        }
        let gains = shared.gains.lock().map(|g| g.clone()).unwrap_or_default();
        mix_block(&taps, &gains, frames, &mut mix, &mut block);
        let peak = mix.iter().fold(0f32, |p, s| p.max(s.abs()));
        let old = f32::from_bits(shared.level.load(Ordering::Relaxed));
        let level = if peak > old { peak } else { old * 0.85 + peak * 0.15 };
        shared.level.store(level.min(1.0).to_bits(), Ordering::Relaxed);
        if let Ok(mut rec) = shared.recording.lock() {
            if let Some(r) = rec.as_mut() {
                r.extend(mix.iter().map(|s| (s * i16::MAX as f32) as i16));
            }
        }
        if let Ok(mut samples) = shared.samples.lock() {
            if let Some(q) = samples.as_mut() {
                q.extend(mix.iter().copied());
                let max = (MIX_RATE * MIX_CHANNELS as u32 * 5) as usize;
                if q.len() > max {
                    let extra = q.len() - max;
                    q.drain(..extra);
                }
            }
        }
        for o in &mut outs {
            let rate = o.queue.rate as f64;
            let target = (0.06 + o.spec.delay_ms as f64 / 1000.0) * rate;
            let queued = o.queue.queued_frames() as f64;
            o.converter.ratio = drift_correction(queued, target, rate);
            if let Ok(mut q) = o.queue.queue.lock() {
                if queued > target + rate * 0.2 {
                    let keep = target as usize * o.queue.channels as usize;
                    let extra = q.len().saturating_sub(keep);
                    q.drain(..extra);
                }
                o.converter.process(&mix, &mut q, o.spec.volume);
            }
        }
    }
}

/// Soma `frames` quadros de cada fonte com os ganhos dela (esquerda, direita) e limita.
fn mix_block(taps: &[Tap], gains: &[(f32, f32)], frames: usize, mix: &mut Vec<f32>, block: &mut Vec<f32>) {
    mix.clear();
    mix.resize(frames * 2, 0.0);
    for (tap, (left, right)) in taps.iter().zip(gains.iter().copied()) {
        tap.take(frames, block);
        if left == 0.0 && right == 0.0 {
            continue;
        }
        for (out, input) in mix.chunks_exact_mut(2).zip(block.chunks_exact(2)) {
            out[0] += input[0] * left;
            out[1] += input[1] * right;
        }
    }
    for s in mix.iter_mut() {
        *s = s.clamp(-1.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mixer_sources() {
        let s = parse_sources("micVoz;volume=1.2 | device:loopback:Alto-falantes;volume=0.4;pan=-2|mpFundo;mute");
        assert_eq!(s.len(), 3);
        assert_eq!(s[0], MixSource { name: "micVoz".into(), device: false, volume: 1.2, pan: 0.0, mute: false });
        assert!(s[1].device);
        assert_eq!(s[1].name, "loopback:Alto-falantes");
        assert_eq!(s[1].pan, -1.0);
        assert!(s[2].mute);
        assert_eq!(s[2].gains(), (0.0, 0.0));
        assert_eq!(s[1].gains(), (0.4 * 1.0, 0.0));
    }

    #[test]
    fn mixes_with_gains_and_limits() {
        let (a, b) = (Tap::new(), Tap::new());
        a.push([0.5, 0.5, 0.5, 0.5]);
        b.push([0.8, -0.2]);
        let (mut mix, mut block) = (Vec::new(), Vec::new());
        mix_block(&[a, b], &[(1.0, 0.0), (1.0, 1.0)], 2, &mut mix, &mut block);
        // quadro 1: a(0.5, 0) + b(0.8, -0.2) = (1.3→1, -0.2); quadro 2: só a, e b em silêncio.
        assert_eq!(mix, vec![1.0, -0.2, 0.5, 0.0]);
    }
}
