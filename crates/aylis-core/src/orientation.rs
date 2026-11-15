//! A orientação da tela no celular (pedido do autor em 14/11), em três níveis:
//!
//! - o **projeto** configura o app todo (`orientation` no `.aylisproj`: o manifesto do Android
//!   e o `Info.plist` do iOS saem com ela, e o app começa com ela, [`Ui::set_app_orientation`]);
//! - o **form** pode ter a sua (a propriedade `orientation`, [`FormOrientation`]; `project`
//!   usa a do app): o form à frente manda, e fechá-lo devolve a do form de baixo;
//! - **em execução** tudo muda: a propriedade do form e o `set_orientation` do `Screen`.
//!
//! O núcleo só guarda o pedido ([`Ui::requested_orientation`]); a plataforma compara com o
//! que aplicou e pede ao sistema quando muda (o Android pelo `setRequestedOrientation`, o iOS
//! pelas orientações válidas da janela). No desktop e no navegador não tem efeito. No
//! desenvolvimento, `AYLIS_ORIENTATION` passa por cima de tudo (só em debug:
//! [`Ui::set_orientation_override`]).

use crate::prop::{PropEnum, PropValue};
use crate::ui::Ui;

/// A orientação que o app pede ao sistema.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ScreenOrientation {
    /// Gira com o aparelho (respeitando a trava de rotação do sistema).
    #[default]
    Any,
    /// Em pé, só no sentido normal.
    Portrait,
    /// Deitado, só num sentido (o botão de início à direita).
    Landscape,
    /// Em pé, nos dois sentidos (o de cabeça para baixo também, onde o aparelho deixa).
    SensorPortrait,
    /// Deitado, nos dois sentidos.
    SensorLandscape,
}

impl ScreenOrientation {
    pub const ALL: [ScreenOrientation; 5] =
        [ScreenOrientation::Any, ScreenOrientation::Portrait, ScreenOrientation::Landscape, ScreenOrientation::SensorPortrait, ScreenOrientation::SensorLandscape];

    /// Pelo nome (`portrait`); vazio é `any`.
    pub fn parse(name: &str) -> Option<ScreenOrientation> {
        let name = name.trim();
        if name.is_empty() { Some(ScreenOrientation::Any) } else { <Self as PropEnum>::from_name(name) }
    }

    /// Fixa em pé (num sentido ou nos dois).
    pub fn is_portrait(self) -> bool {
        matches!(self, ScreenOrientation::Portrait | ScreenOrientation::SensorPortrait)
    }

    /// Fixa deitada (num sentido ou nos dois).
    pub fn is_landscape(self) -> bool {
        matches!(self, ScreenOrientation::Landscape | ScreenOrientation::SensorLandscape)
    }
}

impl PropEnum for ScreenOrientation {
    const NAMES: &'static [&'static str] = &["any", "portrait", "landscape", "sensor_portrait", "sensor_landscape"];
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// A orientação de um form (a propriedade `orientation`): a do app (`project`, o padrão) ou
/// uma própria.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FormOrientation {
    #[default]
    Project,
    Any,
    Portrait,
    Landscape,
    SensorPortrait,
    SensorLandscape,
}

impl FormOrientation {
    pub const ALL: [FormOrientation; 6] = [
        FormOrientation::Project,
        FormOrientation::Any,
        FormOrientation::Portrait,
        FormOrientation::Landscape,
        FormOrientation::SensorPortrait,
        FormOrientation::SensorLandscape,
    ];

    /// A orientação própria do form (`None`: a do app).
    pub fn own(self) -> Option<ScreenOrientation> {
        match self {
            FormOrientation::Project => None,
            FormOrientation::Any => Some(ScreenOrientation::Any),
            FormOrientation::Portrait => Some(ScreenOrientation::Portrait),
            FormOrientation::Landscape => Some(ScreenOrientation::Landscape),
            FormOrientation::SensorPortrait => Some(ScreenOrientation::SensorPortrait),
            FormOrientation::SensorLandscape => Some(ScreenOrientation::SensorLandscape),
        }
    }
}

impl PropEnum for FormOrientation {
    const NAMES: &'static [&'static str] = &["project", "any", "portrait", "landscape", "sensor_portrait", "sensor_landscape"];
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// O que o `Ui` guarda (uma extensão: o `Ui` não ganha campo por isto).
#[derive(Default)]
struct OrientationState {
    app: ScreenOrientation,
    over: Option<ScreenOrientation>,
}

impl Ui {
    /// A orientação do app todo (a do projeto; em execução, o `set_orientation` do `Screen`):
    /// vale nos forms com `orientation = project`.
    pub fn set_app_orientation(&mut self, orientation: ScreenOrientation) {
        if self.app_orientation() != orientation {
            self.extension::<OrientationState>().app = orientation;
            self.request_repaint();
        }
    }

    pub fn app_orientation(&self) -> ScreenOrientation {
        self.extension_ref::<OrientationState>().map_or_else(ScreenOrientation::default, |s| s.app)
    }

    /// Por cima do projeto e dos forms (o `AYLIS_ORIENTATION`, que a plataforma só lê em debug).
    pub fn set_orientation_override(&mut self, orientation: Option<ScreenOrientation>) {
        self.extension::<OrientationState>().over = orientation;
        self.request_repaint();
    }

    /// A orientação própria do form (`None`: `project`, ou o form não tem a propriedade).
    pub fn form_orientation(&self, form: crate::component::ComponentId) -> Option<ScreenOrientation> {
        match self.get_prop(form, "orientation") {
            Ok(PropValue::Enum(name)) => FormOrientation::from_name(&name).and_then(FormOrientation::own),
            _ => None,
        }
    }

    /// O que a plataforma pede ao sistema agora: a variável de desenvolvimento, a do form à
    /// frente (o último aberto) ou a do app. Calculado na hora: mostrar, fechar e mudar a
    /// propriedade já mudam o pedido.
    pub fn requested_orientation(&self) -> ScreenOrientation {
        let state = self.extension_ref::<OrientationState>();
        if let Some(over) = state.and_then(|s| s.over) {
            return over;
        }
        let app = state.map_or_else(ScreenOrientation::default, |s| s.app);
        self.open_forms.last().and_then(|f| self.form_orientation(f.form)).unwrap_or(app)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_names_round_trip() {
        for o in ScreenOrientation::ALL {
            assert_eq!(ScreenOrientation::parse(o.name()), Some(o));
        }
        for o in FormOrientation::ALL {
            assert_eq!(FormOrientation::from_name(o.name()), Some(o));
        }
        assert_eq!(ScreenOrientation::parse(""), Some(ScreenOrientation::Any));
        assert_eq!(ScreenOrientation::parse("sideways"), None);
    }
}
