//! `AudioSpectrum`: o visualizador de áudio (as barras que pulam com a música, a onda, o
//! espelhado), ligado a um `MediaPlayer`, a um `VideoPlayer` ou a um `Microphone`. Decidido em
//! 21/11 (o relatório de um player de música portado para o Aylis: eram 48 painéis, um `Timer`
//! de 40 ms, um `AudioMixer` só para as amostras e a DFT à mão).
//!
//! Um controle só: ele mesmo pinta, com as cores do tema (`primary` nas barras, `text_muted`
//! nos picos; o `accent_color` do controle troca). Ouve antes do volume (o mesmo visualizador em
//! qualquer volume e no mudo). Anima só enquanto há som: com o player parado, as barras descem
//! e os quadros param (a bateria do celular).

use std::cell::RefCell;

use aylis_core::{Brush, Canvas, Component, ComponentId, ControlBase, EventCtx, PaintCtx, PointF, PropEnum, RectF, Stroke, Ui, properties};

use crate::output::{MIX_CHANNELS, MIX_RATE, Tap};

/// Como o espectro aparece.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpectrumStyle {
    /// Barras de baixo para cima.
    #[default]
    Bars,
    /// Barras a partir do meio, para cima e para baixo.
    Mirrored,
    /// A forma da onda (o som no tempo).
    Wave,
}

impl PropEnum for SpectrumStyle {
    const NAMES: &'static [&'static str] = &["bars", "mirrored", "wave"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Bars, Self::Mirrored, Self::Wave].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// As amostras de uma janela da FFT (≈ 21 ms a 48 kHz).
const WINDOW: usize = 1024;

/// O estado entre os quadros: as amostras recentes, a altura de cada barra e os picos.
#[derive(Default)]
struct Live {
    tap: Option<Tap>,
    /// As últimas `WINDOW` amostras (mono).
    recent: Vec<f32>,
    levels: Vec<f32>,
    peaks: Vec<f32>,
    /// O tempo do último quadro (para descer na velocidade certa).
    last: f64,
}

/// O visualizador de áudio.
pub struct AudioSpectrum {
    pub base: ControlBase,
    /// De onde vem o som: um `MediaPlayer`, um `VideoPlayer` ou um `Microphone`.
    pub source: Option<ComponentId>,
    /// Quantas barras (4 a 256).
    pub bands: i64,
    pub style: SpectrumStyle,
    /// Os picos que caem devagar por cima das barras.
    pub peaks: bool,
    /// O vão entre as barras, em pontos.
    pub gap: f32,
    live: RefCell<Live>,
}

impl Default for AudioSpectrum {
    fn default() -> Self {
        Self { base: ControlBase::new(320.0, 96.0), source: None, bands: 48, style: SpectrumStyle::Bars, peaks: true, gap: 2.0, live: RefCell::default() }
    }
}

impl AudioSpectrum {
    /// Liga ao `source` (o ponto de escuta dele, antes do volume).
    fn connect(ui: &mut Ui, id: ComponentId) {
        let Some(source) = ui.get::<AudioSpectrum>(id).and_then(|s| s.source) else {
            if let Some(s) = ui.get::<AudioSpectrum>(id) {
                s.live.borrow_mut().tap = None;
            }
            return;
        };
        if ui.design_mode() {
            return;
        }
        let tap = match ui.class_name(source) {
            Some("MediaPlayer") => crate::MediaPlayer::tap(ui, source, true),
            Some("VideoPlayer") => crate::VideoPlayer::tap(ui, source, true),
            Some("Microphone") => crate::Microphone::tap(ui, source),
            _ => None,
        };
        if let Some(s) = ui.get::<AudioSpectrum>(id) {
            s.live.borrow_mut().tap = tap;
        }
        ui.request_repaint();
    }

    /// Um player começou a tocar: os espectros ligados a ele voltam a animar.
    pub(crate) fn source_started(ui: &mut Ui, source: ComponentId) {
        let ids: Vec<ComponentId> = ui.components().filter(|&c| ui.get::<AudioSpectrum>(c).is_some_and(|s| s.source == Some(source))).collect();
        for id in ids {
            if ui.get::<AudioSpectrum>(id).is_some_and(|s| s.live.borrow().tap.is_none()) {
                Self::connect(ui, id);
            }
            ui.damage(id);
        }
    }

    /// O som está chegando do `source`?
    fn source_active(ui: &Ui, source: Option<ComponentId>) -> bool {
        let Some(s) = source else { return false };
        match ui.class_name(s) {
            Some("MediaPlayer") => crate::MediaPlayer::is_playing(ui, s),
            Some("VideoPlayer") => crate::VideoPlayer::is_playing(ui, s),
            Some("Microphone") => crate::Microphone::level(ui, s) > 0.0 || crate::Microphone::tap(ui, s).is_some(),
            _ => false,
        }
    }

    /// As alturas de agora (0 a 1), uma por barra: para testar e para quem desenha por conta.
    pub fn levels(ui: &Ui, id: ComponentId) -> Vec<f32> {
        ui.get::<AudioSpectrum>(id).map(|s| s.live.borrow().levels.clone()).unwrap_or_default()
    }
}

/// As bandas de frequência das amostras `samples` (mono), em escala logarítmica de 40 Hz a
/// 16 kHz, cada uma de 0 a 1 (em decibéis: -60 dB é 0, 0 dB é 1).
pub fn bands_of(samples: &[f32], bands: usize, rate: u32) -> Vec<f32> {
    let n = samples.len().next_power_of_two().clamp(64, 8192);
    let mut re: Vec<f32> = (0..n).map(|i| samples.get(i).copied().unwrap_or(0.0) * hann(i, n)).collect();
    let mut im = vec![0.0f32; n];
    fft(&mut re, &mut im);
    let mags: Vec<f32> = (0..n / 2).map(|k| (re[k] * re[k] + im[k] * im[k]).sqrt() * 4.0 / n as f32).collect();
    let (lo, hi) = (40.0f32, 16_000.0f32.min(rate as f32 / 2.0));
    let bin = |f: f32| ((f * n as f32 / rate.max(1) as f32) as usize).clamp(1, n / 2 - 1);
    (0..bands)
        .map(|b| {
            let f0 = lo * (hi / lo).powf(b as f32 / bands as f32);
            let f1 = lo * (hi / lo).powf((b + 1) as f32 / bands as f32);
            let (a, z) = (bin(f0), bin(f1).max(bin(f0) + 1));
            let peak = mags[a..z.min(n / 2)].iter().copied().fold(0.0f32, f32::max);
            let db = 20.0 * peak.max(1e-6).log10();
            ((db + 60.0) / 60.0).clamp(0.0, 1.0)
        })
        .collect()
}

fn hann(i: usize, n: usize) -> f32 {
    0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / (n - 1) as f32).cos()
}

/// A FFT de raiz 2, no lugar (`re.len()` potência de 2).
fn fft(re: &mut [f32], im: &mut [f32]) {
    let n = re.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let angle = -std::f32::consts::TAU / len as f32;
        for start in (0..n).step_by(len) {
            for k in 0..len / 2 {
                let (s, c) = (angle * k as f32).sin_cos();
                let (a, b) = (start + k, start + k + len / 2);
                let (tr, ti) = (re[b] * c - im[b] * s, re[b] * s + im[b] * c);
                re[b] = re[a] - tr;
                im[b] = im[a] - ti;
                re[a] += tr;
                im[a] += ti;
            }
        }
        len <<= 1;
    }
}

impl Component for AudioSpectrum {
    fn class_name(&self) -> &'static str {
        "AudioSpectrum"
    }

    fn control(&self) -> Option<&ControlBase> {
        Some(&self.base)
    }

    fn control_mut(&mut self) -> Option<&mut ControlBase> {
        Some(&mut self.base)
    }

    properties! {
        source: Option<ComponentId> => "Behavior",
        bands: i64 => "Appearance",
        style: SpectrumStyle => "Appearance",
        peaks: bool => "Appearance",
        gap: f32 => "Appearance",
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "source" {
            let id = ctx.id;
            ctx.ui.poster().post_to(id, move |ui| AudioSpectrum::connect(ui, id));
        }
        ctx.repaint();
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.source == Some(freed) {
            self.source = None;
            self.live.borrow_mut().tap = None;
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let bands = self.bands.clamp(4, 256) as usize;
        let mut live = self.live.borrow_mut();
        let dt = if live.last > 0.0 { (ctx.time - live.last).clamp(0.0, 0.25) as f32 } else { 0.0 };
        live.last = ctx.time;
        // As amostras novas (estéreo intercalado) viram mono, nas últimas `WINDOW`.
        let mut fresh = false;
        if let Some(tap) = live.tap.clone() {
            let frames = tap.available().min(WINDOW * 4);
            if frames > 0 {
                let mut buf = Vec::new();
                tap.take(frames, &mut buf);
                let ch = MIX_CHANNELS as usize;
                live.recent.extend(buf.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32));
                let extra = live.recent.len().saturating_sub(WINDOW);
                live.recent.drain(..extra);
                fresh = true;
            }
        }
        // No designer (e na galeria), sem som: um desenho de exemplo, para ver as cores e o estilo.
        let target = if fresh {
            bands_of(&live.recent, bands, MIX_RATE)
        } else if ctx.design || live.tap.is_none() && self.source.is_none() {
            (0..bands).map(|i| 0.25 + 0.6 * ((i as f32 * 0.55).sin() * 0.5 + 0.5) * (1.0 - i as f32 / bands as f32 * 0.5)).collect()
        } else {
            vec![0.0; bands]
        };
        if live.levels.len() != bands {
            live.levels = vec![0.0; bands];
            live.peaks = vec![0.0; bands];
        }
        // Sobe na hora, desce devagar (como os medidores); o pico cai mais devagar ainda.
        for i in 0..bands {
            let (l, t) = (live.levels[i], target[i]);
            live.levels[i] = if t > l { t } else { (l - dt * 1.8).max(t) };
            live.peaks[i] = if live.levels[i] >= live.peaks[i] { live.levels[i] } else { (live.peaks[i] - dt * 0.6).max(0.0) };
        }
        let color = ctx.colors.accent.unwrap_or_else(|| ctx.theme.color("primary"));
        let peak_color = ctx.theme.color("text_muted");
        match self.style {
            SpectrumStyle::Wave => {
                let n = live.recent.len().max(2);
                let mid = r.y + r.height / 2.0;
                let mut prev: Option<PointF> = None;
                let step = (n / r.width.max(1.0) as usize).max(1);
                for (k, i) in (0..n).step_by(step).enumerate() {
                    let v = live.recent.get(i).copied().unwrap_or(0.0).clamp(-1.0, 1.0);
                    let p = PointF::new(r.x + k as f32 * step as f32 * r.width / n as f32, mid - v * r.height / 2.0);
                    if let Some(a) = prev {
                        canvas.draw_line(a, p, &Stroke::new(color, 1.5));
                    }
                    prev = Some(p);
                }
                if live.recent.is_empty() {
                    canvas.draw_line(PointF::new(r.x, mid), PointF::new(r.right(), mid), &Stroke::new(color.with_opacity(0.5), 1.0));
                }
            }
            SpectrumStyle::Bars | SpectrumStyle::Mirrored => {
                let w = ((r.width - self.gap * (bands as f32 - 1.0)) / bands as f32).max(1.0);
                let mirrored = self.style == SpectrumStyle::Mirrored;
                for i in 0..bands {
                    let x = r.x + i as f32 * (w + self.gap);
                    let h = (live.levels[i] * r.height).max(1.0);
                    let rect = if mirrored { RectF::new(x, r.y + (r.height - h) / 2.0, w, h) } else { RectF::new(x, r.bottom() - h, w, h) };
                    canvas.fill_rect(rect, (w / 2.0).min(3.0), &Brush::Solid(color));
                    if self.peaks && !mirrored && live.peaks[i] > 0.01 {
                        let y = r.bottom() - live.peaks[i] * r.height;
                        canvas.fill_rect(RectF::new(x, (y - 2.0).max(r.y), w, 2.0), 1.0, &Brush::Solid(peak_color));
                    }
                }
            }
        }
        // Anima enquanto há som ou as barras ainda descem; parado, sem quadros.
        let moving = live.levels.iter().chain(live.peaks.iter()).any(|v| *v > 0.001);
        if !ctx.design && self.source.is_some() && (fresh || moving || Self::source_active(ctx.ui, self.source)) {
            ctx.request_frame();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Um tom de 1 kHz acende as bandas perto de 1 kHz, não as dos graves nem dos agudos.
    #[test]
    fn a_tone_lights_its_band() {
        let tone: Vec<f32> = (0..WINDOW).map(|i| (std::f32::consts::TAU * 1000.0 * i as f32 / MIX_RATE as f32).sin() * 0.8).collect();
        let b = bands_of(&tone, 32, MIX_RATE);
        let loudest = b.iter().enumerate().max_by(|a, z| a.1.total_cmp(z.1)).unwrap().0;
        let f0 = 40.0f32 * (16_000.0f32 / 40.0).powf(loudest as f32 / 32.0);
        let f1 = 40.0f32 * (16_000.0f32 / 40.0).powf((loudest + 1) as f32 / 32.0);
        assert!(f0 <= 1000.0 && 1000.0 <= f1 * 1.2, "a banda {loudest} ({f0}..{f1} Hz)");
        assert!(b[0] < 0.3 && b[31] < 0.3, "{b:?}");
        assert!(bands_of(&[0.0; WINDOW], 16, MIX_RATE).iter().all(|v| *v == 0.0), "silêncio: tudo em 0");
    }
}
