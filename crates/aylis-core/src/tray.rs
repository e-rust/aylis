//! Ícone na bandeja do sistema (perto do relógio): um componente descreve o ícone
//! ([`Component::tray`](crate::Component::tray)) e a plataforma mostra, anima e devolve os
//! cliques como mensagem ([`TrayEvent`]) ao componente.

use std::sync::Arc;

use crate::color::Color;
use crate::component::ComponentId;
use crate::geometry::PointF;
use crate::ui::Ui;

/// O que mostrar na bandeja.
#[derive(Clone, Debug, PartialEq)]
pub struct TraySpec {
    /// Os quadros do ícone (arquivos PNG, ICO... já lidos; a plataforma decodifica). Um
    /// quadro: ícone parado; mais: animação a cada `interval_ms`.
    pub icons: Vec<Arc<Vec<u8>>>,
    pub interval_ms: u32,
    pub tooltip: String,
    /// Indicador sobre o ícone: vazio (nenhum), `•` (um ponto) ou um texto curto (`3`,
    /// `9+`), como os apps de mensagem.
    pub badge: String,
    pub badge_color: Color,
    pub visible: bool,
}

impl Default for TraySpec {
    fn default() -> Self {
        Self { icons: Vec::new(), interval_ms: 0, tooltip: String::new(), badge: String::new(), badge_color: Color::rgb(0xE5, 0x39, 0x35), visible: true }
    }
}

/// O que aconteceu no ícone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayEventKind {
    Click,
    DoubleClick,
    RightClick,
}

/// Mensagem que a plataforma manda ao componente do ícone
/// ([`Ui::send_message`](crate::Ui::send_message)).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrayEvent {
    pub kind: TrayEventKind,
    /// Onde, na tela (pixels físicos): o menu abre ali.
    pub screen: PointF,
    /// Pixels físicos por unidade lógica naquela tela.
    pub scale: f32,
}

impl Ui {
    /// Os ícones de bandeja de todos os componentes (a plataforma sincroniza a cada volta).
    pub fn trays(&self) -> Vec<(ComponentId, TraySpec)> {
        self.components().filter_map(|id| self.component(id).and_then(|c| c.platform_hooks()).and_then(|h| h.tray()).map(|spec| (id, spec))).collect()
    }
}
