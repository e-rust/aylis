//! Quadros por segundo: o limite (para economizar bateria) e a medida. A plataforma conta
//! cada quadro desenhado ([`Ui::frame_drawn`]) e respeita o limite antes de pedir o
//! próximo ([`Ui::next_frame_at`]).

use std::collections::VecDeque;

use crate::ui::Ui;

#[derive(Debug, Default)]
pub(crate) struct FrameClock {
    /// Os instantes (tempo do `Ui`) dos quadros do último segundo.
    times: VecDeque<f64>,
    /// 0 = sem limite.
    max_fps: f32,
    /// Quanto um quadro leva para ser desenhado (média dos últimos, em segundos).
    cost: f64,
}

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

    /// Quando o próximo quadro pode sair (tempo do `Ui`), respeitando o limite. `None` sem
    /// limite.
    pub fn next_frame_at(&self) -> Option<f64> {
        if self.frames.max_fps <= 0.0 {
            return None;
        }
        self.frames.times.back().map(|&last| last + 1.0 / self.frames.max_fps as f64)
    }
}
