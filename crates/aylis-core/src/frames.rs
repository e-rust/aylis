//! Quadros por segundo: o limite (para economizar bateria) e a medida. A plataforma conta
//! cada quadro desenhado ([`Ui::frame_drawn`]) e respeita o limite antes de pedir o
//! próximo ([`Ui::next_frame_at`]).

use std::collections::VecDeque;

use crate::ui::Ui;
use crate::{ComponentId, PropValue};

#[derive(Debug, Default)]
pub(crate) struct FrameClock {
    /// Os instantes (tempo do `Ui`) dos quadros do último segundo.
    times: VecDeque<f64>,
    /// 0 = sem limite.
    max_fps: f32,
    /// Quanto um quadro leva para ser desenhado (média dos últimos, em segundos).
    cost: f64,
    /// Quantas entradas do usuário chegaram (mouse, teclado, roda, toque, o tamanho da janela)
    /// e o tempo da última: o `max_paint_rate` do form nunca segura a pintura delas.
    input_serial: u64,
    input_at: Option<f64>,
    /// Dos forms com `max_paint_rate`: quando pintaram pela última vez e o `input_serial` de
    /// então.
    pub(crate) painted: std::collections::HashMap<crate::ComponentId, (f64, u64)>,
}

/// Depois de uma entrada, por quanto tempo (segundos) a pintura continua sem ritmo: o que a
/// entrada puxa logo depois (o deslize da rolagem, a transição do realce) também é dela.
pub const INPUT_GRACE: f64 = 0.25;

impl Ui {
    /// Limita os quadros por segundo (0 = sem limite, o padrão: acompanha a tela).
    /// Animações continuam suaves dentro do limite; parado, o aylis já não desenha nada.
    /// Útil no celular e em notebooks: `ui.set_max_fps(30.0)` corta pela metade o trabalho
    /// de uma animação contínua.
    pub fn set_max_fps(&mut self, fps: f32) {
        self.frames.max_fps = if fps.is_finite() && fps > 0.0 { fps } else { 0.0 };
    }

    pub fn max_fps(&self) -> f32 {
        self.frames.max_fps
    }

    /// Quantos quadros foram desenhados no último segundo (0 com a tela parada, porque o
    /// aylis só desenha quando algo muda).
    pub fn fps(&self) -> f32 {
        let now = self.time;
        self.frames.times.iter().filter(|&&t| now - t <= 1.0).count() as f32
    }

    /// A plataforma avisa que desenhou um quadro (no tempo atual do `Ui`).
    pub fn frame_drawn(&mut self) {
        let now = self.time;
        self.frames.times.push_back(now);
        while self.frames.times.front().is_some_and(|&t| now - t > 1.0) {
            self.frames.times.pop_front();
        }
    }

    /// A plataforma diz quanto o último quadro levou para ser desenhado (em segundos).
    pub fn frame_cost_measured(&mut self, seconds: f64) {
        let c = &mut self.frames.cost;
        *c = if *c == 0.0 { seconds } else { *c * 0.8 + seconds * 0.2 };
    }

    /// Quanto um quadro leva para ser desenhado (a média dos últimos; 0 antes do primeiro).
    pub fn frame_cost(&self) -> f64 {
        self.frames.cost
    }

    /// A rolagem pode deslizar: o deslize está ligado e os quadros são rápidos (até 20 ms;
    /// com quadros lentos, como num build de depuração, o deslize atrasaria a roda e ela
    /// salta direto).
    pub fn smooth_scrolling(&self) -> bool {
        self.scroll_options().smooth && self.frames.cost <= 0.020
    }

    /// Quando o último quadro foi desenhado (tempo do `Ui`).
    pub fn last_frame(&self) -> Option<f64> {
        self.frames.times.back().copied()
    }

    /// Uma entrada do usuário chegou (a plataforma e as entradas do `Ui` avisam): as pinturas
    /// dela e as do [`INPUT_GRACE`] seguinte não esperam o `max_paint_rate` do form.
    pub fn note_input(&mut self) {
        self.frames.input_serial += 1;
        self.frames.input_at = Some(self.time);
    }

    /// O ritmo máximo de pintura do form `form` (a propriedade `max_paint_rate`; 0 = sem
    /// ritmo, cada mudança pinta na próxima volta).
    pub fn form_max_paint_rate(&self, form: ComponentId) -> f64 {
        match self.get_prop(form, "max_paint_rate") {
            Ok(PropValue::Float(rate)) if rate.is_finite() && rate > 0.0 => rate,
            _ => 0.0,
        }
    }

    /// Até quando (tempo do `Ui`) a pintura do form `form` espera: com o `max_paint_rate`
    /// ligado, o que mudou sem entrada do usuário (um quadro de vídeo, um timer, uma animação)
    /// junta numa pintura só, no máximo `max_paint_rate` vezes por segundo. `None`: pode
    /// pintar já (sem ritmo, com entrada nova ou recente, ou já deu a vez).
    pub fn paint_held_until(&self, form: ComponentId) -> Option<f64> {
        let rate = self.form_max_paint_rate(form);
        if rate <= 0.0 {
            return None;
        }
        let &(last, serial) = self.frames.painted.get(&form)?;
        if serial != self.frames.input_serial || self.frames.input_at.is_some_and(|at| self.time - at < INPUT_GRACE) {
            return None;
        }
        let at = last + 1.0 / rate;
        (at > self.time + 0.0005).then_some(at)
    }

    /// O form `form` foi pintado agora ([`Ui::take_frame`]): a próxima pintura sem entrada
    /// conta o `max_paint_rate` daqui.
    pub(crate) fn form_painted(&mut self, form: ComponentId) {
        if self.form_max_paint_rate(form) > 0.0 {
            self.frames.painted.insert(form, (self.time, self.frames.input_serial));
        } else if !self.frames.painted.is_empty() {
            self.frames.painted.remove(&form);
        }
    }

    /// A vez do primeiro form que espera o `max_paint_rate` com algo para pintar (tempo do
    /// `Ui`). `None` com nada pendente: parado, nenhum relógio.
    pub(crate) fn next_paced_paint(&self) -> Option<f64> {
        if self.frames.painted.is_empty() || !self.needs_repaint() {
            return None;
        }
        self.frames.painted.keys().filter(|&&f| self.contains(f)).filter_map(|&f| self.paint_held_until(f)).reduce(f64::min)
    }

    /// Quando o próximo quadro pode sair (tempo do `Ui`), respeitando o limite. `None` sem
    /// limite.
    pub fn next_frame_at(&self) -> Option<f64> {
        if self.frames.max_fps <= 0.0 {
            return None;
        }
        self.frames.times.back().map(|&last| last + 1.0 / self.frames.max_fps as f64)
    }
}
