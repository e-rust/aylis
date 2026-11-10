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

    fn pending_closes_from_window(&mut self, form: ComponentId) {
        // Como o X da janela: o form principal encerra o app.
        if self.main_form() == Some(form) && self.open_forms().len() <= 1 {
            self.request_close();
        } else {
            self.close(form);
        }
    }
}
