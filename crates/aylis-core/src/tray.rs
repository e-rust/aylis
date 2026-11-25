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
    /// O GUID fixo do ícone (o `NIF_GUID` do Windows), ou `None`. O Windows lembra o lugar do
    /// ícone (na barra ou no "^") por ele, preso ao caminho do exe.
    pub guid: Option<u128>,
    /// O `uID` fixo do ícone no Windows; 0 = o que a plataforma escolher. Sem GUID, o Windows
    /// lembra o lugar do ícone pelo caminho do exe mais este número.
    pub uid: u32,
}

/// Lê o texto de um GUID (`{0F8A...-...}`, com ou sem chaves e hífens) como o número de 128
/// bits na ordem do texto (a do `GUID::from_u128` do Windows). Vazio ou inválido: `None`.
pub fn parse_guid(text: &str) -> Option<u128> {
    let t = text.trim();
    let t = t.strip_prefix('{').and_then(|t| t.strip_suffix('}')).unwrap_or(t);
    let hyphenated = t.len() == 36 && [8, 13, 18, 23].iter().all(|&i| t.as_bytes()[i] == b'-');
    let hex: String = if hyphenated { t.chars().filter(|&c| c != '-').collect() } else { t.to_string() };
    if hex.len() != 32 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    u128::from_str_radix(&hex, 16).ok()
}

impl Default for TraySpec {
    fn default() -> Self {
        Self { icons: Vec::new(), interval_ms: 0, tooltip: String::new(), badge: String::new(), badge_color: Color::rgb(0xE5, 0x39, 0x35), visible: true, guid: None, uid: 0 }
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

#[cfg(test)]
mod tests {
    use super::parse_guid;

    #[test]
    fn guid_text_is_read_in_text_order() {
        let n = 0x0F8A_1B2C_3D4E_5F60_7182_93A4_B5C6_D7E8u128;
        assert_eq!(parse_guid("{0F8A1B2C-3D4E-5F60-7182-93A4B5C6D7E8}"), Some(n));
        assert_eq!(parse_guid("0f8a1b2c-3d4e-5f60-7182-93a4b5c6d7e8"), Some(n));
        assert_eq!(parse_guid(" 0F8A1B2C3D4E5F60718293A4B5C6D7E8 "), Some(n));
        assert_eq!(parse_guid(""), None);
        assert_eq!(parse_guid("0F8A1B2C-3D4E-5F60-7182"), None);
        assert_eq!(parse_guid("0F8A1B2C-3D4E-5F6-07182-93A4B5C6D7E8"), None);
        assert_eq!(parse_guid("{0F8A1B2C-3D4E-5F60-7182-93A4B5C6D7EZ}"), None);
    }
}
