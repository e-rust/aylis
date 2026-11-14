//! A barra de status e a de navegação do celular (decidido em 14/11).

use aylis_core::{BarIcons, Color, Component, ComponentId, EventCtx, SystemBarsStyle, Ui, properties};

/// Escolhe as barras do sistema no celular: a cor da barra de status (`status_bar_color`) e a
/// da de navegação (`navigation_bar_color`; vazia, a mesma da de status), a cor dos ícones
/// (`icons`: `auto` pela claridade da barra, `light` ou `dark`) e se cada uma aparece
/// (`status_bar_visible`, `navigation_bar_visible`: escondidas, a tela fica cheia, e a área
/// segura muda). Sem cor, as barras ficam com o fundo do form principal, como sem o
/// componente. O pedido vale para o app todo (com mais de um, o último que mudou).
///
/// | Plataforma | O que faz |
/// |---|---|
/// | Android | pinta as duas barras, escolhe os ícones e esconde (com o `targetSdk` 35, o Android 15 ignora a cor; os ícones e o esconder continuam) |
/// | iOS | os ícones e o esconder pelo sistema; a cor é uma faixa que o Aylis pinta por baixo da barra de status (não há barra de navegação) |
/// | Navegador | a cor vai para a `theme-color` da página (a barra do navegador no celular) |
/// | Desktop | nada (`available` falso) |
pub struct SystemBars {
    /// A cor da barra de status (em cima); vazia: o fundo do form principal.
    pub status_bar_color: Option<Color>,
    /// A cor da barra de navegação (embaixo); vazia: a mesma da de status.
    pub navigation_bar_color: Option<Color>,
    /// A cor dos ícones das barras: `auto` (pela claridade da barra), `light` ou `dark`.
    pub icons: BarIcons,
    /// A barra de status aparece; escondida, a tela fica cheia e a área segura muda.
    pub status_bar_visible: bool,
    /// A barra de navegação aparece; escondida, a tela fica cheia e a área segura muda.
    pub navigation_bar_visible: bool,
}

impl Default for SystemBars {
    fn default() -> Self {
        Self { status_bar_color: None, navigation_bar_color: None, icons: BarIcons::Auto, status_bar_visible: true, navigation_bar_visible: true }
    }
}

impl SystemBars {
    /// O pedido que este componente faz.
    pub fn style(&self) -> SystemBarsStyle {
        SystemBarsStyle {
            status_color: self.status_bar_color,
            navigation_color: self.navigation_bar_color,
            icons: self.icons,
            status_visible: self.status_bar_visible,
            navigation_visible: self.navigation_bar_visible,
        }
    }

    /// Aplica o pedido de `id` (depois de o form carregar, com todas as propriedades postas).
    pub fn apply(ui: &mut Ui, id: ComponentId) {
        if let Some(style) = ui.get::<SystemBars>(id).map(SystemBars::style) {
            ui.set_system_bars(Some(style));
        }
    }

    /// As barras do sistema aplicam o pedido aqui (o Android, o iOS, o navegador).
    fn available(&self) -> bool {
        aylis_core::system_bars::available()
    }

    fn set_b(&mut self, _: bool) {}
}

impl Component for SystemBars {
    fn class_name(&self) -> &'static str {
        "SystemBars"
    }

    properties! {
        status_bar_color: Option<Color> => "Appearance",
        navigation_bar_color: Option<Color> => "Appearance",
        icons: BarIcons => "Appearance",
        status_bar_visible: bool => "Behavior",
        navigation_bar_visible: bool => "Behavior";
        computed: available: bool => available / set_b
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "available" || ctx.ui.design_mode() {
            return;
        }
        // Depois do form carregar: as propriedades chegam uma a uma, e o pedido vai inteiro.
        let id = ctx.id;
        ctx.ui.set_timeout(0, move |ui| SystemBars::apply(ui, id));
    }
}
