//! As configurações da janela que só alguns sistemas têm, num componente só (pedido do autor em
//! 16/11): pôr no form e configurar nele, valer para o app inteiro, ou trocar pelo código, como
//! qualquer propriedade. Onde o sistema não tem o recurso, a propriedade existe e não faz nada.

use aylis_core::{Color, Component, EventCtx, WindowBackdrop, WindowCommand, properties};

/// A quem o [`WindowSettings`] vale.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WindowSettingsScope {
    /// A janela do form em que ele está.
    #[default]
    Form,
    /// As janelas de todos os forms do app (o de um form ganha deste).
    App,
}

impl aylis_core::prop::PropEnum for WindowSettingsScope {
    const NAMES: &'static [&'static str] = &["form", "app"];
    fn from_index(i: usize) -> Option<Self> {
        [WindowSettingsScope::Form, WindowSettingsScope::App].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Configura a janela do form em que está (ou, com `scope` = `app`, as de todos os forms):
/// animação, captura, clique, botões da barra, fundo e cores do sistema, sombra, desfoque,
/// foco e Alt+Tab. A plataforma aplica ao abrir a janela e a cada troca em execução (a
/// janela recebe as opções de novo); no designer, nada muda na IDE. Lido pela plataforma em
/// [`aylis_core::Ui::window_options`].
pub struct WindowSettings {
    /// A quem vale: `form` (a janela do form em que está) ou `app` (todas as janelas; o de um
    /// form ganha deste).
    pub scope: WindowSettingsScope,
    /// A animação do Windows ao abrir, minimizar e restaurar (`false`: aparece e some na hora,
    /// como o editor de um print sobre a tela congelada).
    pub animation: bool,
    /// Fora das capturas de tela e das gravações (Windows e macOS): a janela sai preta ou some
    /// no que outro programa grava.
    pub capture_excluded: bool,
    /// O mouse atravessa a janela: os cliques vão para o que está atrás (um HUD por cima).
    pub click_through: bool,
    /// O botão minimizar da barra do sistema (Windows e macOS).
    pub minimize_button: bool,
    /// O botão maximizar da barra do sistema (Windows e macOS).
    pub maximize_button: bool,
    /// O fundo que o Windows 11 desenha por trás do conteúdo: `system`, `none`, `mica`,
    /// `acrylic` ou `tabbed` (aparece onde o form não pinta fundo).
    pub backdrop: WindowBackdrop,
    /// A cor da barra de título do sistema (Windows 11; vazio: a do sistema).
    pub title_color: Option<Color>,
    /// A cor do texto da barra de título do sistema (Windows 11; vazio: a do sistema).
    pub title_text_color: Option<Color>,
    /// A cor da borda da janela (Windows 11; vazio: a do sistema).
    pub border_color: Option<Color>,
    /// A sombra do Windows na janela sem borda (`border_style` = `none`).
    pub shadow: bool,
    /// O que está atrás da janela sai desfocado (Windows, macOS e Linux com KDE). Com o form
    /// transparente (`transparent`, `window_shape`) e o `fill` com alfa, é o vidro fosco; no
    /// Windows, o desfoque segue a forma do form, com a borda suave.
    pub blur: bool,
    /// Clicar na janela não tira o foco de onde ele está (Windows): um teclado na tela, uma
    /// paleta flutuante.
    pub no_activate: bool,
    /// A janela aparece no Alt+Tab (Windows; desligado, também sai da barra de tarefas).
    pub alt_tab: bool,
    /// Um número livre para o código (não muda nada).
    pub tag: i64,
}

impl Default for WindowSettings {
    fn default() -> Self {
        let d = aylis_core::WindowOptions::default();
        Self {
            scope: WindowSettingsScope::Form,
            animation: d.animation,
            capture_excluded: d.capture_excluded,
            click_through: d.click_through,
            minimize_button: d.minimize_button,
            maximize_button: d.maximize_button,
            backdrop: d.backdrop,
            title_color: d.title_color,
            title_text_color: d.title_text_color,
            border_color: d.border_color,
            shadow: d.shadow,
            blur: d.blur,
            no_activate: d.no_activate,
            alt_tab: d.alt_tab,
            tag: 0,
        }
    }
}

impl Component for WindowSettings {
    fn class_name(&self) -> &'static str {
        aylis_core::WINDOW_SETTINGS_CLASS
    }

    properties! {
        scope: WindowSettingsScope => "Behavior",
        animation: bool => "Behavior",
        capture_excluded: bool => "Behavior",
        click_through: bool => "Behavior",
        minimize_button: bool => "Behavior",
        maximize_button: bool => "Behavior",
        backdrop: WindowBackdrop => "Appearance",
        title_color: Option<Color> => "Appearance",
        title_text_color: Option<Color> => "Appearance",
        border_color: Option<Color> => "Appearance",
        shadow: bool => "Appearance",
        blur: bool => "Appearance",
        no_activate: bool => "Behavior",
        alt_tab: bool => "Behavior",
        tag: i64 => "Misc",
    }

    /// Com o app rodando, as janelas recebem as opções de novo (no designer não há janela).
    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, _name: &str) {
        if !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, WindowCommand::UpdateOptions);
        }
    }
}
