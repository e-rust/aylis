//! Comandos à janela de um form (minimizar, maximizar, arrastar...): o que uma barra de
//! título própria precisa. O `Ui` só enfileira; a plataforma executa no laço dela.

use crate::component::ComponentId;
use crate::ui::Ui;

/// A borda (ou canto) por onde a janela é redimensionada.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResizeEdge {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Um pedido à janela do form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowCommand {
    Minimize,
    Maximize,
    Restore,
    /// Maximiza ou volta (o duplo clique na barra de título).
    ToggleMaximize,
    /// Fecha o form (passa pelo `on_close_query`, como o X).
    Close,
    /// Começa a arrastar a janela com o botão do mouse (chame no `pointer_down`).
    StartDrag,
    /// Começa a redimensionar pela borda (chame no `pointer_down`).
    StartResize(ResizeEdge),
    /// Sempre por cima das outras janelas.
    SetTopmost(bool),
    /// Esconde a janela sem fechar o form (o app continua, como ir para a bandeja).
    Hide,
    /// Mostra a janela escondida (ou minimizada) e a traz para a frente.
    Show,
    /// Tela cheia (sem bordas, no monitor da janela) ou de volta.
    SetFullscreen(bool),
    /// Leva a janela para o monitor (índice em [`Ui::monitors`](crate::Ui::monitors)), no meio
    /// da área útil dele.
    MoveToMonitor(usize),
    /// Põe o canto da janela (com a moldura) neste ponto da tela, em pixels físicos (a janela
    /// arrastada pela aba que vai para outro lugar).
    MoveTo(i32, i32),
    /// Põe a janela (com a moldura) no lugar e no tamanho dados em pixels físicos, com a
    /// origem no canto do monitor `monitor` (índice em [`Ui::monitors`](crate::Ui::monitors);
    /// fora da lista, a área de trabalho inteira). Maximizada ou em tela cheia, volta ao normal
    /// antes. `activate`: traz a janela para a frente com o foco; sem ele, nem o foco nem a
    /// ordem mudam (o painel de avisos no canto, sempre por cima).
    Place { monitor: usize, x: i32, y: i32, width: u32, height: u32, activate: bool },
    /// A propriedade `gpu` do form mudou: a janela confere em que placa de vídeo desenha
    /// (veja [`crate::gpu`]).
    UpdateGpu,
    /// A janela fica com a área cliente do tamanho do form (`width`/`height`, ou
    /// `client_width`/`client_height`, mudados com o app rodando): a moldura do sistema fica
    /// por fora, sem o app descontar a borda (o relatório do Tabernáculo, J11).
    ApplySize,
    /// A propriedade `window_corners` (ou a `border_style`) do form mudou: a janela pede ao
    /// sistema os cantos dela de novo.
    UpdateCorners,
    /// A propriedade `span_monitors` do form mudou: a janela confere se o form vira uma
    /// janela por monitor (veja [`crate::span`]).
    UpdateSpan,
    /// A propriedade `physical_pixels` do form mudou: a janela (ou os pedaços dela) passa a
    /// desenhar com 1 unidade = 1 pixel físico, ou volta à escala do monitor (veja
    /// [`Ui::physical_pixels`]).
    UpdateScale,
    /// O `WindowSettings` do form (ou o do app) mudou: a janela aplica as opções de novo
    /// (veja [`WindowOptions`]).
    UpdateOptions,
    /// Chama a atenção para a janela sem tomar o foco: no Windows, o botão dela pisca na barra
    /// de tarefas até o usuário ativá-la (`true`, urgente: a janela pisca também); no macOS, o
    /// ícone pula no Dock (urgente: até ativar); no Linux, a dica de urgência do gerenciador.
    RequestAttention(bool),
    /// O `topmost` ou o `bottommost` do form mudou: a janela fica por cima de todas, por baixo
    /// de todas ou normal.
    UpdateLevel,
    /// O `keep_awake` de um form mudou (ou um form abriu ou fechou): a plataforma junta o de
    /// todos os forms abertos e pede ao sistema.
    UpdateKeepAwake,
    /// O `confine_cursor` do form mudou: o mouse fica preso na janela, no monitor dela ou solto.
    UpdateCursorConfine,
    /// O `window_opacity` do form mudou: a janela inteira (com a moldura) fica translúcida.
    UpdateOpacity,
}

impl Ui {
    /// Pede algo à janela do form `form` (ou do form de um controle dentro dele).
    pub fn window_command(&mut self, form: ComponentId, command: WindowCommand) {
        let form = self.root_of(form);
        if command == WindowCommand::Close {
            self.pending_closes_from_window(form);
            return;
        }
        self.window_commands.push((form, command));
        self.request_repaint();
    }

    /// A janela do form (ou do form de um controle) vem para a frente pelo caminho forçado
    /// (a propriedade `force_foreground`): no Windows, só o processo que recebeu a última
    /// entrada pode pôr uma janela em primeiro plano, e o form aberto por um atalho global
    /// (o hook de teclado), por um timer ou pela bandeja apareceria atrás, sem o teclado. A
    /// plataforma usa ao mostrar ([`WindowCommand::Show`], ao abrir) e ao ativar. No designer,
    /// nunca.
    pub fn forces_foreground(&self, form: ComponentId) -> bool {
        let form = self.root_of(form);
        !self.design_mode() && matches!(self.get_prop(form, "force_foreground"), Ok(crate::PropValue::Bool(true)))
    }

    /// O form (a janela) com que o usuário falou por último: o ativado, clicado ou que
    /// recebeu uma tecla (sempre o form raiz, nunca um popup dele); `None` antes da primeira
    /// vez ou depois de ele fechar.
    pub fn active_form(&self) -> Option<ComponentId> {
        self.active_form.filter(|&f| self.is_open(f))
    }

    /// Os pedidos à janela desde a última vez (a plataforma chama a cada volta do laço).
    pub fn take_window_commands(&mut self) -> Vec<(ComponentId, WindowCommand)> {
        std::mem::take(&mut self.window_commands)
    }

    /// A janela do form está maximizada (a plataforma informa; ver
    /// [`WindowPlacement`](crate::forms::WindowPlacement)).
    pub fn is_maximized(&self, form: ComponentId) -> bool {
        self.window_placement(self.root_of(form)).is_some_and(|p| p.maximized)
    }

    /// O botão maximizar de uma barra de título própria (`owner`, a barra), no lugar dela
    /// (`None`: não há mais). A plataforma o entrega ao sistema como o botão maximizar da
    /// janela: no Windows 11, o mouse parado nele abre o menu de layouts do sistema (Snap).
    pub fn set_maximize_button(&mut self, owner: ComponentId, rect: Option<crate::geometry::RectF>) {
        let form = self.root_of(owner);
        match rect {
            Some(r) => {
                self.maximize_buttons.insert(form, (owner, r));
            }
            None => {
                if self.maximize_buttons.get(&form).is_some_and(|(o, _)| *o == owner) {
                    self.maximize_buttons.remove(&form);
                }
            }
        }
    }

    /// Onde fica o botão maximizar do form `form` (relativo ao form, do último layout): o da
    /// barra própria que o informou, se ela continua no form e à vista.
    pub fn maximize_button(&self, form: ComponentId) -> Option<crate::geometry::RectF> {
        let (owner, local) = *self.maximize_buttons.get(&form)?;
        let shown = self.cb(owner).is_some_and(|cb| cb.visible()) && self.shown_rect(owner).is_some();
        if !self.contains(owner) || self.root_of(owner) != form || !shown {
            return None;
        }
        let at = self.absolute_rect(owner)?;
        Some(local.offset(at.x, at.y))
    }

    fn pending_closes_from_window(&mut self, form: ComponentId) {
        // Como o X da janela: o form principal encerra o app.
        if self.main_form() == Some(form) && self.open_forms().len() <= 1 {
            self.request_close();
        } else {
            self.close(form);
        }
    }
}

/// O fundo da janela que o sistema desenha por trás do conteúdo (Windows 11): o do sistema, sem
/// fundo, o Mica, o Acrylic ou o Mica das janelas com abas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WindowBackdrop {
    #[default]
    System,
    None,
    Mica,
    Acrylic,
    Tabbed,
}

impl crate::prop::PropEnum for WindowBackdrop {
    const NAMES: &'static [&'static str] = &["system", "none", "mica", "acrylic", "tabbed"];
    fn from_index(i: usize) -> Option<Self> {
        [WindowBackdrop::System, WindowBackdrop::None, WindowBackdrop::Mica, WindowBackdrop::Acrylic, WindowBackdrop::Tabbed].get(i).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// A cor da borda da janela (o `border_color` do `WindowSettings`; Windows 11): a do sistema,
/// nenhuma (sem a linha de 1 px, também em volta da tela cheia) ou uma cor. No form, vazio, o
/// texto `none` ou a cor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BorderColor {
    /// A do sistema (a cor de destaque, se o usuário ligou; senão, cinza).
    #[default]
    System,
    /// Sem borda (o `DWMWA_COLOR_NONE` do Windows).
    None,
    Color(crate::Color),
}

impl crate::prop::PropType for BorderColor {
    const KIND: crate::prop::PropKind = crate::prop::PropKind::Color;
    fn to_value(&self) -> crate::PropValue {
        match self {
            BorderColor::System => crate::PropValue::Null,
            BorderColor::None => crate::PropValue::String("none".into()),
            BorderColor::Color(c) => crate::PropValue::Color(*c),
        }
    }
    fn from_value(value: crate::PropValue) -> crate::error::Result<Self> {
        match value {
            crate::PropValue::Null => Ok(BorderColor::System),
            crate::PropValue::String(s) if s.trim().is_empty() || s.trim().eq_ignore_ascii_case("system") => Ok(BorderColor::System),
            crate::PropValue::String(s) if s.trim().eq_ignore_ascii_case("none") => Ok(BorderColor::None),
            v => v.to_color().map(BorderColor::Color),
        }
    }
}

impl From<crate::Color> for BorderColor {
    fn from(c: crate::Color) -> Self {
        BorderColor::Color(c)
    }
}

/// No acesso tipado (`set_border_color(ui, BorderColor::None)`).
impl From<BorderColor> for crate::handles::ColorRef {
    fn from(b: BorderColor) -> Self {
        match b {
            BorderColor::System => crate::handles::ColorRef::token(""),
            BorderColor::None => crate::handles::ColorRef::token("none"),
            BorderColor::Color(c) => c.into(),
        }
    }
}

/// O nome da classe do componente que configura a janela ([`WindowOptions`]).
pub const WINDOW_SETTINGS_CLASS: &str = "WindowSettings";

/// O que o componente `WindowSettings` configura na janela de um form: os recursos que só
/// alguns sistemas têm, num lugar só (pedido do autor em 16/11). Onde o sistema não tem o
/// recurso, o valor não faz nada.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowOptions {
    /// A animação do sistema ao abrir, minimizar e restaurar (Windows).
    pub animation: bool,
    /// Fora das capturas de tela e das gravações (Windows e macOS).
    pub capture_excluded: bool,
    /// O mouse atravessa a janela (os cliques vão para o que está atrás).
    pub click_through: bool,
    /// Os botões minimizar e maximizar da barra do sistema (Windows e macOS).
    pub minimize_button: bool,
    pub maximize_button: bool,
    /// O fundo do sistema por trás do conteúdo (Windows 11).
    pub backdrop: WindowBackdrop,
    /// As cores da barra de título do sistema, do texto dela e da borda (Windows 11; `None`:
    /// as do sistema).
    pub title_color: Option<crate::Color>,
    pub title_text_color: Option<crate::Color>,
    pub border_color: BorderColor,
    /// A sombra do sistema na janela sem borda (Windows).
    pub shadow: bool,
    /// O que está atrás da janela sai desfocado (Windows, macOS e Linux com KDE). Com o form
    /// transparente (`transparent`, `window_shape`) e o fundo com alfa, é o vidro fosco; no Windows,
    /// o desfoque segue a forma do form, com a borda suave.
    pub blur: bool,
    /// Clicar na janela não tira o foco de onde ele está (Windows).
    pub no_activate: bool,
    /// A janela aparece no Alt+Tab (Windows; `false` também a tira da barra de tarefas).
    pub alt_tab: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            animation: true,
            capture_excluded: false,
            click_through: false,
            minimize_button: true,
            maximize_button: true,
            backdrop: WindowBackdrop::System,
            title_color: None,
            title_text_color: None,
            border_color: BorderColor::System,
            shadow: false,
            blur: false,
            no_activate: false,
            alt_tab: true,
        }
    }
}

impl Ui {
    /// O `WindowSettings` que vale para a janela do form `form`: o que está no próprio form
    /// (`scope` = `form`) ou, sem ele, um com `scope` = `app` em qualquer form ou DataModule.
    pub fn window_settings_of(&self, form: ComponentId) -> Option<ComponentId> {
        let is_settings = |id: &&ComponentId| self.class_name(**id) == Some(WINDOW_SETTINGS_CLASS);
        let app_scope = |id: ComponentId| matches!(self.get_prop(id, "scope"), Ok(crate::PropValue::Enum(s)) if s == "app");
        self.children(form)
            .iter()
            .filter(is_settings)
            .find(|id| !app_scope(**id))
            .or_else(|| self.roots().iter().flat_map(|r| self.children(*r).iter()).filter(is_settings).find(|id| app_scope(**id)))
            .copied()
    }

    /// As opções da janela do form `form` ([`Ui::window_settings_of`]; sem componente, o padrão
    /// do sistema).
    pub fn window_options(&self, form: ComponentId) -> WindowOptions {
        let Some(id) = self.window_settings_of(form) else { return WindowOptions::default() };
        let d = WindowOptions::default();
        let flag = |name: &str, default: bool| match self.get_prop(id, name) {
            Ok(crate::PropValue::Bool(b)) => b,
            _ => default,
        };
        let color = |name: &str| match self.get_prop(id, name) {
            Ok(crate::PropValue::Color(c)) => Some(c),
            _ => None,
        };
        let backdrop = match self.get_prop(id, "backdrop") {
            Ok(crate::PropValue::Enum(name)) => <WindowBackdrop as crate::prop::PropEnum>::from_name(&name).unwrap_or_default(),
            _ => WindowBackdrop::System,
        };
        WindowOptions {
            animation: flag("animation", d.animation),
            capture_excluded: flag("capture_excluded", d.capture_excluded),
            click_through: flag("click_through", d.click_through),
            minimize_button: flag("minimize_button", d.minimize_button),
            maximize_button: flag("maximize_button", d.maximize_button),
            backdrop,
            title_color: color("title_color"),
            title_text_color: color("title_text_color"),
            border_color: match self.get_prop(id, "border_color") {
                Ok(v) => <BorderColor as crate::prop::PropType>::from_value(v).unwrap_or_default(),
                Err(_) => BorderColor::System,
            },
            shadow: flag("shadow", d.shadow),
            blur: flag("blur", d.blur),
            no_activate: flag("no_activate", d.no_activate),
            alt_tab: flag("alt_tab", d.alt_tab),
        }
    }
}
