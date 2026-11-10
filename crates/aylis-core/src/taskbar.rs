//! O botão do app na barra de tarefas: progresso, indicador, botões na miniatura (ao passar
//! o mouse) e tarefas no clique direito (a "jump list" do Windows). Um componente descreve
//! ([`Component::taskbar`](crate::Component::taskbar)); a plataforma aplica no botão da
//! janela do form e devolve os cliques nos botões como [`TaskbarEvent`].

use std::sync::Arc;

use crate::color::Color;
use crate::component::ComponentId;
use crate::ui::Ui;

/// Estado da barra de progresso no botão.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TaskbarProgress {
    #[default]
    None,
    Normal,
    Paused,
    Error,
    /// Sem valor: anda sozinha.
    Indeterminate,
}

impl crate::prop::PropEnum for TaskbarProgress {
    const NAMES: &'static [&'static str] = &["none", "normal", "paused", "error", "indeterminate"];

    fn from_index(index: usize) -> Option<Self> {
        [Self::None, Self::Normal, Self::Paused, Self::Error, Self::Indeterminate].get(index).copied()
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// Um botão da miniatura.
#[derive(Clone, Debug, PartialEq)]
pub struct ThumbButton {
    /// O ícone (arquivo PNG/ICO já lido; a plataforma decodifica).
    pub icon: Option<Arc<Vec<u8>>>,
    pub tooltip: String,
}

/// Uma tarefa do clique direito: abre o próprio app com estes argumentos.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JumpTask {
    pub title: String,
    pub arguments: String,
}

/// O que mostrar no botão da barra de tarefas.
#[derive(Clone, Debug, PartialEq)]
pub struct TaskbarSpec {
    pub progress: TaskbarProgress,
    /// 0 a 100.
    pub value: f32,
    /// Indicador sobre o ícone: vazio, `•` ou um número.
    pub badge: String,
    pub badge_color: Color,
    pub buttons: Vec<ThumbButton>,
    pub tasks: Vec<JumpTask>,
}

impl Default for TaskbarSpec {
    fn default() -> Self {
        Self {
            progress: TaskbarProgress::None,
            value: 0.0,
            badge: String::new(),
            badge_color: Color::rgb(0xE5, 0x39, 0x35),
            buttons: Vec::new(),
            tasks: Vec::new(),
        }
    }
}

/// Mensagem que a plataforma manda ao componente: um botão da miniatura foi clicado.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TaskbarEvent {
    pub button: usize,
}

impl Ui {
    /// Os componentes de barra de tarefas, com o form (a janela) de cada um.
    pub fn taskbars(&self) -> Vec<(ComponentId, ComponentId, TaskbarSpec)> {
        self.components()
            .filter_map(|id| self.component(id).and_then(|c| c.platform_hooks()).and_then(|h| h.taskbar()).map(|spec| (id, self.root_of(id), spec)))
            .collect()
    }
}
