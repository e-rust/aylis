//! Alinhamento e âncoras dos controles dentro do pai.

use std::ops::BitOr;

use crate::prop::PropEnum;

/// Alinhamento de um controle dentro do pai.
///
/// Os alinhamentos de encaixe (`MostTop`…`Right`) consomem espaço da área livre do pai,
/// nesta ordem: `MostTop`, `MostBottom`, `MostLeft`, `MostRight`, `Top`, `Bottom`,
/// `Left`, `Right`. Controles com o mesmo alinhamento seguem a ordem dos filhos.
/// Os demais usam a área que sobrou.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Align {
    /// Posição e tamanho livres; obedece às [`Anchors`].
    #[default]
    None,
    Top,
    Bottom,
    Left,
    Right,
    MostTop,
    MostBottom,
    MostLeft,
    MostRight,
    /// Ocupa toda a área que sobrou.
    Client,
    /// Ocupa o pai inteiro, ignorando o padding e os outros controles.
    Contents,
    /// Mantém o tamanho e centraliza na área que sobrou.
    Center,
    /// Centraliza na vertical e estica na horizontal.
    VertCenter,
    /// Centraliza na horizontal e estica na vertical.
    HorzCenter,
    /// Estica na horizontal; mantém `top` e `height`.
    Horizontal,
    /// Estica na vertical; mantém `left` e `width`.
    Vertical,
    /// Escala mantendo a proporção até caber, centralizado.
    Fit,
}

impl Align {
    pub const ALL: [Align; 17] = [
        Align::None,
        Align::Top,
        Align::Bottom,
        Align::Left,
        Align::Right,
        Align::MostTop,
        Align::MostBottom,
        Align::MostLeft,
        Align::MostRight,
        Align::Client,
        Align::Contents,
        Align::Center,
        Align::VertCenter,
        Align::HorzCenter,
        Align::Horizontal,
        Align::Vertical,
        Align::Fit,
    ];

    /// Verdadeiro para os alinhamentos que consomem espaço da área livre.
    pub fn is_dock(self) -> bool {
        matches!(
            self,
            Align::Top
                | Align::Bottom
                | Align::Left
                | Align::Right
                | Align::MostTop
                | Align::MostBottom
                | Align::MostLeft
                | Align::MostRight
        )
    }
}

impl PropEnum for Align {
    const NAMES: &'static [&'static str] = &[
        "none",
        "top",
        "bottom",
        "left",
        "right",
        "most_top",
        "most_bottom",
        "most_left",
        "most_right",
        "client",
        "contents",
        "center",
        "vert_center",
        "horz_center",
        "horizontal",
        "vertical",
        "fit",
    ];

    fn from_index(index: usize) -> Option<Self> {
        Align::ALL.get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Onde um controle encaixado fica no sentido em que o encaixe estica (como o `align-self`
/// do CSS): em cima ou embaixo, na horizontal; à esquerda ou à direita, na vertical.
///
/// `Stretch` (o padrão) ocupa a faixa inteira (com a `max_width`, no meio). Os outros mantêm
/// a própria `width` (ou `height`) e encostam no começo, no meio ou no fim: um botão ou um
/// fundo do tamanho do conteúdo numa pilha de campos, sem conta de margem pela largura do pai.
/// Com a `max_width` (ou `max_height`), ocupam a faixa até a máxima e encostam do mesmo
/// jeito: uma coluna de leitura à esquerda numa tela larga.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SelfAlign {
    #[default]
    Stretch,
    Start,
    Center,
    End,
}

impl SelfAlign {
    pub const ALL: [SelfAlign; 4] = [SelfAlign::Stretch, SelfAlign::Start, SelfAlign::Center, SelfAlign::End];
}

impl PropEnum for SelfAlign {
    const NAMES: &'static [&'static str] = &["stretch", "start", "center", "end"];

    fn from_index(index: usize) -> Option<Self> {
        SelfAlign::ALL.get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Lados do pai aos quais um controle com `Align::None` fica preso.
///
/// - esquerda + direita: a largura acompanha o pai;
/// - só direita: o controle se move junto com a borda direita;
/// - nenhum dos dois: o controle se move metade do quanto o pai cresceu.
///
/// O mesmo vale para cima/embaixo. O padrão é esquerda + cima.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Anchors(u8);

impl Anchors {
    pub const NONE: Anchors = Anchors(0);
    pub const LEFT: Anchors = Anchors(1);
    pub const TOP: Anchors = Anchors(1 << 1);
    pub const RIGHT: Anchors = Anchors(1 << 2);
    pub const BOTTOM: Anchors = Anchors(1 << 3);
    pub const ALL: Anchors = Anchors(0b1111);

    /// Nomes na mesma ordem dos bits.
    pub const NAMES: [&'static str; 4] = ["left", "top", "right", "bottom"];

    pub const fn contains(self, other: Anchors) -> bool {
        self.0 & other.0 == other.0
    }

    pub const fn union(self, other: Anchors) -> Anchors {
        Anchors(self.0 | other.0)
    }

    pub fn names(self) -> Vec<String> {
        Self::NAMES
            .iter()
            .enumerate()
            .filter(|(i, _)| self.0 & (1 << i) != 0)
            .map(|(_, n)| n.to_string())
            .collect()
    }

    /// Monta a partir de nomes (`"left"`, `"right"`...). `None` se algum nome for inválido.
    pub fn from_names<I, S>(names: I) -> Option<Anchors>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut bits = 0u8;
        for name in names {
            let i = Self::NAMES.iter().position(|n| n.eq_ignore_ascii_case(name.as_ref()))?;
            bits |= 1 << i;
        }
        Some(Anchors(bits))
    }
}

impl Default for Anchors {
    fn default() -> Self {
        Anchors::LEFT | Anchors::TOP
    }
}

impl BitOr for Anchors {
    type Output = Anchors;
    fn bitor(self, other: Anchors) -> Anchors {
        self.union(other)
    }
}
