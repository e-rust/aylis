//! Taskbar: o botão do app na barra de tarefas do Windows. Progresso, indicador (número,
//! como os apps de mensagem), botões na miniatura (ao passar o mouse, como os de tocar e
//! pausar dos players) e tarefas no clique direito.

use std::any::Any;
use std::cell::RefCell;
use std::sync::Arc;

use aylis_core::{Color, Component, EventArgs, EventCtx, JumpTask, TaskbarEvent, TaskbarProgress, TaskbarSpec, ThumbButton, properties};

/// O botão do form na barra de tarefas. Não visual: fica no form, e vale para a janela dele.
pub struct Taskbar {
    /// A barra de progresso no botão: `none` (sem barra), `normal` (verde), `paused`
    /// (amarela), `error` (vermelha) ou `indeterminate` (anda sozinha, sem valor).
    pub progress: TaskbarProgress,
    /// 0 a 100.
    pub value: f32,
    /// Indicador sobre o ícone: vazio, `•` ou um número.
    pub badge: String,
    /// A cor do fundo do indicador.
    pub badge_color: Color,
    /// Botões da miniatura (até 7), em JSON: `[{"icon": "play.png", "hint": "Tocar"}]`.
    pub buttons: String,
    /// Tarefas do clique direito, em JSON: `[{"name": "Nova janela", "arguments": "--new"}]`.
    /// Abrem o próprio app com os argumentos (veja `std::env::args`).
    pub tasks: String,
    icons: RefCell<(String, Vec<Option<Arc<Vec<u8>>>>)>,
}

impl Default for Taskbar {
    fn default() -> Self {
        Self {
            progress: TaskbarProgress::None,
            value: 0.0,
            badge: String::new(),
            badge_color: Color::rgb(0xE5, 0x39, 0x35),
            buttons: String::new(),
            tasks: String::new(),
            icons: RefCell::default(),
        }
    }
}

/// Um botão da miniatura no JSON do `buttons` (o [`THUMB_BUTTON`]).
#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct ThumbButtonRecord {
    icon: String,
    hint: String,
}

/// Uma tarefa no JSON do `tasks` (o [`JUMP_TASK`]).
#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct JumpTaskRecord {
    name: String,
    arguments: String,
}

/// Um botão da miniatura do `Taskbar`: o arquivo do ícone (PNG) e a dica.
pub const THUMB_BUTTON: aylis_core::RecordInfo = aylis_core::RecordInfo { name: "ThumbButton", fields: &[("icon", aylis_core::PropKind::String), ("hint", aylis_core::PropKind::String)] };

/// Uma tarefa do clique direito do `Taskbar`: o nome e os argumentos com que o app abre.
pub const JUMP_TASK: aylis_core::RecordInfo = aylis_core::RecordInfo { name: "JumpTask", fields: &[("name", aylis_core::PropKind::String), ("arguments", aylis_core::PropKind::String)] };

impl Component for Taskbar {
    fn class_name(&self) -> &'static str {
        "Taskbar"
    }

    properties! {
        progress: TaskbarProgress => "Behavior",
        value: f32 => "Behavior",
        badge: String => "Appearance",
        badge_color: Color => "Appearance",
        buttons: String as RecordList(&THUMB_BUTTON) => "Behavior",
        tasks: String as RecordList(&JUMP_TASK) => "Behavior";
        events: on_button: i64
    }

    fn platform_hooks(&self) -> Option<&dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn platform_hooks_mut(&mut self) -> Option<&mut dyn aylis_core::PlatformHooks> {
        Some(self)
    }

    fn message(&mut self, ctx: &mut EventCtx<'_>, message: &dyn Any) {
        if let Some(e) = message.downcast_ref::<TaskbarEvent>() {
            ctx.fire("on_button", EventArgs::Value((e.button as i64).into()));
        }
    }
}

impl aylis_core::PlatformHooks for Taskbar {
    fn taskbar(&self) -> Option<TaskbarSpec> {
        let buttons: Vec<ThumbButtonRecord> = crate::records::parse_list(&self.buttons);
        if self.icons.borrow().0 != self.buttons {
            let icons = buttons.iter().map(|b| std::fs::read(b.icon.trim()).ok().map(Arc::new)).collect();
            *self.icons.borrow_mut() = (self.buttons.clone(), icons);
        }
        let icons = self.icons.borrow();
        Some(TaskbarSpec {
            progress: self.progress,
            value: self.value.clamp(0.0, 100.0),
            badge: self.badge.clone(),
            badge_color: self.badge_color,
            buttons: buttons
                .into_iter()
                .take(7)
                .enumerate()
                .map(|(i, b)| ThumbButton { icon: icons.1.get(i).cloned().flatten(), tooltip: b.hint })
                .collect(),
            tasks: crate::records::parse_list::<JumpTaskRecord>(&self.tasks)
                .into_iter()
                .filter(|t| !t.name.trim().is_empty())
                .map(|t| JumpTask { title: t.name, arguments: t.arguments })
                .collect(),
        })
    }
}
