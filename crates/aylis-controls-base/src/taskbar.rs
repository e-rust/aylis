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
    /// Botões da miniatura (até 7): `icone.png|Dica` separados por `;`.
    pub buttons: String,
    /// Tarefas do clique direito: `Nome|argumentos` separados por `;`. Abrem o próprio app
    /// com os argumentos (veja `std::env::args`).
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

fn pairs(text: &str) -> Vec<(String, String)> {
    text.split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (a, b) = s.split_once('|').unwrap_or((s, ""));
            (a.trim().to_string(), b.trim().to_string())
        })
        .collect()
}

impl Component for Taskbar {
    fn class_name(&self) -> &'static str {
        "Taskbar"
    }

    properties! {
        progress: TaskbarProgress => "Behavior",
        value: f32 => "Behavior",
        badge: String => "Appearance",
        badge_color: Color => "Appearance",
        buttons: String => "Behavior",
        tasks: String => "Behavior";
        events: on_button
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
        let buttons = pairs(&self.buttons);
        if self.icons.borrow().0 != self.buttons {
            let icons = buttons.iter().map(|(file, _)| std::fs::read(file).ok().map(Arc::new)).collect();
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
                .map(|(i, (_, tooltip))| ThumbButton { icon: icons.1.get(i).cloned().flatten(), tooltip })
                .collect(),
            tasks: pairs(&self.tasks).into_iter().map(|(title, arguments)| JumpTask { title, arguments }).collect(),
        })
    }
}
