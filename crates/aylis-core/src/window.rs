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
