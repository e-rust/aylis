//! Saídas de áudio: a lista do sistema ([`output_devices`]), a escolha de uma ou várias
//! (`output` nos players: [`parse_outputs`]) e a fila que alimenta cada uma ([`Sink`]).
//!
//! Várias saídas tocam o mesmo som sincronizado: cada uma tem a sua fila e o seu conversor,
//! e o conversor corrige devagar a taxa pela fila (o relógio de cada placa anda um pouco
//! diferente). `volume` e `delay` (ms) por saída calibram, por exemplo, um fone Bluetooth.
//!
//! Android: escolhe a saída, mas só usa a primeira da lista.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Uma saída de áudio do sistema.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputDevice {
    pub name: String,
    /// A saída padrão do sistema.
    pub is_default: bool,
    pub channels: u16,
    pub sample_rate: u32,
}

/// As saídas de áudio do sistema (a padrão marcada).
pub fn output_devices() -> Vec<OutputDevice> {
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();
    let default = host.default_output_device().and_then(|d| d.description().ok()).map(|d| d.name().to_string());
    let Ok(devices) = host.output_devices() else { return Vec::new() };
    devices
        .filter_map(|d| {
            let name = d.description().ok()?.name().to_string();
            let config = d.default_output_config().ok()?;
            Some(OutputDevice { is_default: default.as_deref() == Some(name.as_str()), name, channels: config.channels(), sample_rate: config.sample_rate() })
        })
        .collect()
}

/// O registro de uma saída no acesso tipado (a lista do `output` dos players, do `Sound`, do
/// `AudioMixer` e o `monitor` do `Microphone`): `[{"name": "Fone BT", "volume": 0.8,
/// "delay_ms": 180}]`.
pub const OUTPUT_SPEC_RECORD: aylis_core::RecordInfo = aylis_core::RecordInfo {
    name: "OutputSpec",
    fields: &[("name", aylis_core::PropKind::String), ("volume", aylis_core::PropKind::Float), ("delay_ms", aylis_core::PropKind::Float)],
};

/// Uma saída escolhida, com o volume (0 a 2) e o atraso (ms) dela. Nome vazio: a padrão.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputSpec {
    pub name: String,
    pub volume: f32,
    pub delay_ms: f32,
}

impl OutputSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), volume: 1.0, delay_ms: 0.0 }
    }
}

/// Lê a propriedade `output`: a lista de [`OUTPUT_SPEC_RECORD`] em JSON, cada saída com o
/// `volume` (sem ele, 1) e o `delay_ms` opcionais. Vazio, `[]` ou JSON inválido: só a saída
/// padrão.
///
/// `[{"name": "Alto-falantes"}, {"name": "Fone BT", "volume": 0.8, "delay_ms": 180}]`
pub fn parse_outputs(text: &str) -> Vec<OutputSpec> {
    let items = match serde_json::from_str::<serde_json::Value>(text) {
        Ok(serde_json::Value::Array(items)) => items,
        _ => Vec::new(),
    };
    let number = |j: &serde_json::Value| j.as_f64().map(|v| v as f32).filter(|v| v.is_finite());
    let mut specs: Vec<OutputSpec> = items
        .iter()
        .filter(|j| j.is_object())
        .map(|j| {
            let mut spec = OutputSpec::new(j["name"].as_str().unwrap_or_default().trim());
            if let Some(v) = number(&j["volume"]) {
                spec.volume = v.clamp(0.0, 2.0);
            }
            if let Some(v) = number(&j["delay_ms"]) {
                spec.delay_ms = v.clamp(0.0, 5_000.0);
            }
            spec
        })
        .collect();
    if specs.is_empty() {
        specs.push(OutputSpec::new(""));
    }
    // Android: o sistema toca numa saída por vez.
    if cfg!(target_os = "android") {
        specs.truncate(1);
    }
    specs
}

/// Escreve a lista no formato de [`parse_outputs`] (só o que difere do padrão).
pub fn format_outputs(specs: &[OutputSpec]) -> String {
    let items: Vec<serde_json::Value> = specs
        .iter()
        .map(|s| {
            let mut o = serde_json::Map::new();
            o.insert("name".into(), s.name.clone().into());
            if s.volume != 1.0 {
                o.insert("volume".into(), serde_json::json!(s.volume as f64));
            }
            if s.delay_ms != 0.0 {
                o.insert("delay_ms".into(), serde_json::json!(s.delay_ms as f64));
            }
            serde_json::Value::Object(o)
        })
        .collect();
    serde_json::Value::Array(items).to_string()
}

/// A fila de uma saída aberta: quem toca põe amostras (no formato dela); o sistema tira.
pub struct SinkQueue {
    pub queue: Mutex<VecDeque<f32>>,
    /// Tira da fila (pausado: silêncio, e a fila fica como está).
    pub playing: AtomicBool,
    /// Quadros tocados desde [`reset`](Self::reset).
    pub played: AtomicU64,
    pub rate: u32,
    pub channels: u16,
}

impl SinkQueue {
    /// Quantos quadros esperam na fila.
    pub fn queued_frames(&self) -> usize {
        self.queue.lock().map_or(0, |q| q.len()) / self.channels.max(1) as usize
    }

    /// Esvazia a fila, zera o contador e põe `delay_ms` de silêncio na frente.
    pub fn reset(&self, delay_ms: f32) {
        if let Ok(mut q) = self.queue.lock() {
            q.clear();
            let frames = (self.rate as f32 * delay_ms / 1000.0) as usize;
            q.extend(std::iter::repeat_n(0.0, frames * self.channels as usize));
        }
        self.played.store(0, Ordering::Relaxed);
    }
}

/// Uma saída aberta. O fluxo do sistema vive na thread que abriu (não é `Send` em todo
/// sistema); a fila é compartilhada.
pub struct Sink {
    _stream: cpal::Stream,
    pub queue: Arc<SinkQueue>,
}

/// Procura a saída pelo nome (vazio: a padrão).
pub(crate) fn find_output(name: &str) -> Option<cpal::Device> {
    use cpal::traits::{DeviceTrait, HostTrait};
    let host = cpal::default_host();
    if name.is_empty() {
        return host.default_output_device();
    }
    host.output_devices().ok()?.find(|d| d.description().is_ok_and(|n| n.name() == name))
}

impl Sink {
    /// Abre a saída `name` (vazio: a padrão do sistema), começando pausada.
    pub fn open(name: &str) -> Result<Sink, String> {
        use cpal::traits::{DeviceTrait, StreamTrait};
        let Some(device) = find_output(name) else {
            return Err(if name.is_empty() { aylis_core::messages::t("nenhuma saída de áudio") } else { format!("{}: {name}", aylis_core::messages::t("saída de áudio não encontrada")) });
        };
        let config = device.default_output_config().map_err(|e| format!("saída de áudio: {e}"))?;
        let queue = Arc::new(SinkQueue {
            queue: Mutex::new(VecDeque::new()),
            playing: AtomicBool::new(false),
            played: AtomicU64::new(0),
            rate: config.sample_rate(),
            channels: config.channels(),
        });
        let stream_config: cpal::StreamConfig = config.clone().into();
        let channels = config.channels().max(1) as usize;
        // O callback do sistema tira amostras da fila (silêncio se faltar ou estiver pausado).
        let fill = {
            let shared = queue.clone();
            move |out: &mut dyn FnMut(usize, f32), len: usize| {
                let playing = shared.playing.load(Ordering::Relaxed);
                let mut taken = 0;
                if let Ok(mut q) = shared.queue.lock() {
                    for i in 0..len {
                        let s = if playing { q.pop_front().inspect(|_| taken += 1).unwrap_or(0.0) } else { 0.0 };
                        out(i, s);
                    }
                } else {
                    (0..len).for_each(|i| out(i, 0.0));
                }
                shared.played.fetch_add((taken / channels) as u64, Ordering::Relaxed);
            }
        };
        let error = |e| aylis_core::stderr_line!("aylis-media: erro de áudio: {e}");
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                stream_config,
                move |d: &mut [f32], _| {
                    let n = d.len();
                    fill(&mut |i, s| d[i] = s, n)
                },
                error,
                None,
            ),
            cpal::SampleFormat::I16 => device.build_output_stream(
                stream_config,
                move |d: &mut [i16], _| {
                    let n = d.len();
                    fill(&mut |i, s| d[i] = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16, n)
                },
                error,
                None,
            ),
            cpal::SampleFormat::U16 => device.build_output_stream(
                stream_config,
                move |d: &mut [u16], _| {
                    let n = d.len();
                    fill(&mut |i, s| d[i] = ((s.clamp(-1.0, 1.0) * 0.5 + 0.5) * u16::MAX as f32) as u16, n)
                },
                error,
                None,
            ),
            other => return Err(format!("formato de saída {other:?} não suportado")),
        };
        let stream = stream.map_err(|e| format!("saída de áudio: {e}"))?;
        stream.play().map_err(|e| format!("saída de áudio: {e}"))?;
        Ok(Sink { _stream: stream, queue })
    }
}

/// O fator de correção da taxa de uma saída pela fila dela: fila acima do alvo (a placa
/// anda mais devagar) produz um pouco menos; abaixo, um pouco mais. No máximo 0,5%.
pub(crate) fn drift_correction(queued_frames: f64, target_frames: f64, rate: f64) -> f64 {
    let error = (queued_frames - target_frames) / rate.max(1.0); // segundos
    1.0 + (error * 0.02).clamp(-0.005, 0.005)
}

/// Um ponto de escuta: recebe uma cópia do som (48 kHz, estéreo) de um player ou de uma
/// entrada, para o [`AudioMixer`](crate::AudioMixer). Guarda no máximo 1 s.
#[derive(Clone, Default)]
pub struct Tap {
    inner: Arc<Mutex<VecDeque<f32>>>,
}

/// O formato dos pontos de escuta e do mixer.
pub const MIX_RATE: u32 = 48_000;
pub const MIX_CHANNELS: u16 = 2;

impl Tap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Acrescenta amostras (48 kHz, estéreo); passando de 1 s, joga fora as mais velhas.
    pub fn push(&self, samples: impl IntoIterator<Item = f32>) {
        if let Ok(mut q) = self.inner.lock() {
            q.extend(samples);
            let max = (MIX_RATE * MIX_CHANNELS as u32) as usize;
            if q.len() > max {
                let extra = q.len() - max;
                q.drain(..extra);
            }
        }
    }

    /// Tira até `frames` quadros (completa com silêncio).
    pub fn take(&self, frames: usize, out: &mut Vec<f32>) {
        out.clear();
        let n = frames * MIX_CHANNELS as usize;
        if let Ok(mut q) = self.inner.lock() {
            let available = q.len().min(n);
            out.extend(q.drain(..available));
        }
        out.resize(n, 0.0);
    }

    /// Quadros esperando.
    pub fn available(&self) -> usize {
        self.inner.lock().map_or(0, |q| q.len()) / MIX_CHANNELS as usize
    }

    /// Ninguém mais escuta (só o dono ficou com ele).
    pub(crate) fn orphan(&self) -> bool {
        Arc::strong_count(&self.inner) == 1
    }

    /// Uma referência que não segura o ponto de escuta: quem produz o som guarda esta, para
    /// religar o mesmo ponto quando troca o player de dentro (o `MediaPlayer` com outro arquivo).
    pub(crate) fn downgrade(&self) -> WeakTap {
        WeakTap(Arc::downgrade(&self.inner))
    }
}

/// Um [`Tap`] guardado sem segurá-lo ([`Tap::downgrade`]): some quando quem escuta solta.
#[derive(Clone)]
pub(crate) struct WeakTap(std::sync::Weak<Mutex<VecDeque<f32>>>);

impl WeakTap {
    pub(crate) fn upgrade(&self) -> Option<Tap> {
        self.0.upgrade().map(|inner| Tap { inner })
    }
}

/// Os pontos de escuta de quem produz som: converte para o formato do mixer e entrega.
#[derive(Default)]
pub(crate) struct Taps {
    /// O ponto, o conversor e se ele ouve antes do volume (`pre_volume`).
    list: Mutex<Vec<(Tap, crate::audio::Converter, bool)>>,
}

impl Taps {
    pub(crate) fn add(&self, rate: u32, channels: u16) -> Tap {
        let tap = Tap::new();
        self.attach(tap.clone(), rate, channels, false);
        tap
    }

    /// Liga um ponto que já existe; `pre_volume`: recebe o som sem o volume de quem produz.
    pub(crate) fn attach(&self, tap: Tap, rate: u32, channels: u16, pre_volume: bool) {
        if let Ok(mut l) = self.list.lock() {
            l.push((tap, crate::audio::Converter::new(rate, channels, MIX_RATE, MIX_CHANNELS), pre_volume));
        }
    }

    /// Entrega `input` (na taxa e nos canais de quem produz) a cada ponto de escuta.
    pub(crate) fn feed(&self, input: &[f32], rate: u32, channels: u16) {
        self.feed_with_volume(input, rate, channels, 1.0);
    }

    /// Como [`Taps::feed`], com o volume de quem produz: os pontos `pre_volume` recebem o som
    /// sem ele (o visualizador e o medidor iguais em qualquer volume e no mudo; o relatório do
    /// player de música, 21/11), os outros, com ele.
    pub(crate) fn feed_with_volume(&self, input: &[f32], rate: u32, channels: u16, volume: f32) {
        let Ok(mut l) = self.list.lock() else { return };
        l.retain(|(t, _, _)| !t.orphan());
        let mut buffer = VecDeque::new();
        for (tap, converter, pre) in l.iter_mut() {
            if converter.in_rate != rate || converter.in_channels != channels.max(1) as usize {
                *converter = crate::audio::Converter::new(rate, channels, MIX_RATE, MIX_CHANNELS);
            }
            buffer.clear();
            converter.process(input, &mut buffer, if *pre { 1.0 } else { volume });
            tap.push(buffer.drain(..));
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.list.lock().map_or(true, |l| l.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_outputs_with_volume_and_delay() {
        let specs = parse_outputs(r#"[{"name": "Alto-falantes"}, {"name": "Fone BT", "volume": 0.8, "delay_ms": 180}]"#);
        assert_eq!(specs.len(), 2);
        assert_eq!(specs[0], OutputSpec::new("Alto-falantes"));
        assert_eq!(specs[1].name, "Fone BT");
        assert_eq!(specs[1].volume, 0.8);
        assert_eq!(specs[1].delay_ms, 180.0);
        assert_eq!(parse_outputs(&format_outputs(&specs)), specs, "ida e volta");
        assert_eq!(parse_outputs(""), vec![OutputSpec::new("")]);
        assert_eq!(parse_outputs("[]"), vec![OutputSpec::new("")]);
        assert_eq!(parse_outputs("Fone BT"), vec![OutputSpec::new("")], "JSON inválido: a padrão");
        assert_eq!(parse_outputs(r#"[{"name": "X", "volume": 9, "delay_ms": "abc"}]"#)[0].volume, 2.0);
    }

    #[test]
    fn drift_correction_is_small_and_signed() {
        assert_eq!(drift_correction(4800.0, 4800.0, 48_000.0), 1.0);
        assert!(drift_correction(9600.0, 4800.0, 48_000.0) > 1.0);
        assert!(drift_correction(0.0, 4800.0, 48_000.0) < 1.0);
        assert!(drift_correction(1e9, 0.0, 48_000.0) <= 1.005);
    }

    #[test]
    fn tap_keeps_at_most_one_second() {
        let tap = Tap::new();
        tap.push(std::iter::repeat_n(0.5, 3 * MIX_RATE as usize * 2));
        assert_eq!(tap.available(), MIX_RATE as usize);
        let mut out = Vec::new();
        tap.take(10, &mut out);
        assert_eq!(out, vec![0.5; 20]);
        let empty = Tap::new();
        empty.take(3, &mut out);
        assert_eq!(out, vec![0.0; 6]);
    }

    #[test]
    fn taps_convert_to_the_mixer_format() {
        let taps = Taps::default();
        let tap = taps.add(24_000, 1);
        taps.feed(&vec![0.25; 2400], 24_000, 1);
        // 2400 quadros a 24 kHz ≈ 4800 a 48 kHz, em estéreo.
        assert!((tap.available() as i64 - 4800).abs() <= 2, "{}", tap.available());
        drop(tap);
        taps.feed(&[0.0; 10], 24_000, 1);
        assert!(taps.is_empty());
    }
}
