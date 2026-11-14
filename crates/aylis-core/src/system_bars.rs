//! As barras do sistema no celular (a de status e a de navegação), pedidas pelo componente
//! `SystemBars` (decidido em 14/11). O `Ui` guarda o pedido e a plataforma aplica: o Android
//! pinta as barras e esconde; o iOS escolhe a cor dos ícones e esconde (a cor da barra no iOS
//! é o que o app desenha por baixo dela: a faixa que o núcleo pinta, [`Ui::paints_status_strip`]);
//! o navegador põe a cor na `theme-color` da página. Sem pedido, a plataforma faz o de sempre
//! (no Android, as barras com o fundo do form principal).

use std::sync::atomic::{AtomicBool, Ordering};

use crate::color::Color;
use crate::ui::Ui;

/// A cor dos ícones da barra (relógio, bateria).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BarIcons {
    /// Pela claridade da cor da barra (escuros sobre fundo claro).
    #[default]
    Auto,
    /// Ícones claros (para barra escura).
    Light,
    /// Ícones escuros (para barra clara).
    Dark,
}

impl crate::prop::PropEnum for BarIcons {
    const NAMES: &'static [&'static str] = &["auto", "light", "dark"];
    fn from_index(i: usize) -> Option<Self> {
        [BarIcons::Auto, BarIcons::Light, BarIcons::Dark].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// O que o app pede às barras (sem cor: a do fundo do form, como sem pedido).
#[derive(Clone, Debug, PartialEq)]
pub struct SystemBarsStyle {
    pub status_color: Option<Color>,
    /// Sem cor: a da barra de status.
    pub navigation_color: Option<Color>,
    pub icons: BarIcons,
    pub status_visible: bool,
    pub navigation_visible: bool,
}

impl Default for SystemBarsStyle {
    fn default() -> Self {
        Self { status_color: None, navigation_color: None, icons: BarIcons::Auto, status_visible: true, navigation_visible: true }
    }
}

impl SystemBarsStyle {
    /// Os ícones escuros (para uma barra de cor `background`)?
    pub fn dark_icons(&self, background: Color) -> bool {
        match self.icons {
            BarIcons::Dark => true,
            BarIcons::Light => false,
            BarIcons::Auto => 0.299 * background.r as f32 + 0.587 * background.g as f32 + 0.114 * background.b as f32 > 140.0,
        }
    }
}

/// A plataforma tem barras que o app escolhe (o Android, o iOS, o navegador).
static AVAILABLE: AtomicBool = AtomicBool::new(false);

/// A plataforma diz que aplica o pedido das barras (o `available` do `SystemBars`).
pub fn set_available(available: bool) {
    AVAILABLE.store(available, Ordering::Relaxed);
}

/// As barras do sistema aplicam o pedido nesta plataforma.
pub fn available() -> bool {
    AVAILABLE.load(Ordering::Relaxed)
}

impl Ui {
    /// O pedido das barras (o `SystemBars`; o último aplicado vale para o app todo).
    pub fn set_system_bars(&mut self, style: Option<SystemBarsStyle>) {
        if self.system_bars != style {
            self.system_bars = style;
            self.system_bars_version += 1;
            self.request_repaint();
        }
    }

    /// O pedido das barras, se houver.
    pub fn system_bars(&self) -> Option<&SystemBarsStyle> {
        self.system_bars.as_ref()
    }

    /// Muda a cada pedido novo: a plataforma aplica quando muda.
    pub fn system_bars_version(&self) -> u64 {
        self.system_bars_version
    }

    /// Onde a barra de status não tem cor própria (o iOS), o núcleo pinta a faixa dela (a área
    /// segura de cima do form principal) com a cor pedida.
    pub fn set_paints_status_strip(&mut self, paints: bool) {
        self.paints_status_strip = paints;
    }

    pub fn paints_status_strip(&self) -> bool {
        self.paints_status_strip
    }

    /// A faixa da barra de status a pintar por cima de `root` (nas coordenadas dele), com a cor.
    pub(crate) fn status_strip(&self, root: crate::component::ComponentId) -> Option<(crate::geometry::RectF, Color)> {
        if !self.paints_status_strip || self.main_form() != Some(root) {
            return None;
        }
        let style = self.system_bars.as_ref().filter(|s| s.status_visible)?;
        let color = style.status_color?;
        let cb = self.cb(root)?;
        let top = cb.safe_area.top;
        (top > 0.0).then(|| (crate::geometry::RectF::new(0.0, 0.0, cb.bounds().width, top), color))
    }
}
