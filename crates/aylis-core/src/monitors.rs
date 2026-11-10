//! Os monitores (como o `Screen.Monitors` do Delphi): a plataforma informa a lista
//! ([`Ui::set_monitors`]) e em qual deles está a janela de cada form; o app lê com
//! [`Ui::monitors`] e [`Ui::form_monitor`] (nas outras linguagens, pelo componente `Screen`).
//! Coordenadas em pixels físicos da área de trabalho, como o sistema dá (o monitor à esquerda
//! do principal tem `x` negativo).

use crate::component::ComponentId;
use crate::forms::WindowPlacement;
use crate::ui::Ui;

/// Um modo de vídeo que o monitor aceita.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VideoMode {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u16,
    pub refresh_hz: f64,
}

/// Um monitor.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Monitor {
    /// O nome do aparelho ("XB273K V5"); sem ele, o do sistema.
    pub name: String,
    /// O nome do sistema (`\\.\DISPLAY1` no Windows): não muda enquanto o monitor fica ligado
    /// na mesma saída, por isso é o que se guarda para reabrir nele.
    pub device: String,
    pub primary: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    /// A área útil (sem a barra de tarefas); igual à do monitor onde o sistema não diz.
    pub work_x: i32,
    pub work_y: i32,
    pub work_width: u32,
    pub work_height: u32,
    /// A escala do sistema (1,5 = 150%).
    pub scale: f64,
    /// A taxa de atualização (0 = o sistema não diz, como no navegador).
    pub refresh_hz: f64,
    pub modes: Vec<VideoMode>,
}

impl Monitor {
    /// A área útil, em pixels físicos: `(x, y, largura, altura)`.
    pub fn work_area(&self) -> (i32, i32, u32, u32) {
        (self.work_x, self.work_y, self.work_width, self.work_height)
    }

    /// Em JSON (o `Screen` das outras linguagens), com os nomes dos campos em inglês.
    pub fn to_json(&self, index: usize) -> String {
        let text = |s: &str| {
            let mut out = String::from("\"");
            for c in s.chars() {
                match c {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                    c => out.push(c),
                }
            }
            out.push('"');
            out
        };
        let modes: Vec<String> = self
            .modes
            .iter()
            .map(|m| format!("{{\"width\":{},\"height\":{},\"bit_depth\":{},\"refresh_hz\":{}}}", m.width, m.height, m.bit_depth, m.refresh_hz))
            .collect();
        format!(
            "{{\"index\":{index},\"name\":{},\"device\":{},\"primary\":{},\"x\":{},\"y\":{},\"width\":{},\"height\":{},\"work_x\":{},\"work_y\":{},\"work_width\":{},\"work_height\":{},\"scale\":{},\"refresh_hz\":{},\"modes\":[{}]}}",
            text(&self.name),
            text(&self.device),
            self.primary,
            self.x,
            self.y,
            self.width,
            self.height,
            self.work_x,
            self.work_y,
            self.work_width,
            self.work_height,
            self.scale,
            self.refresh_hz,
            modes.join(",")
        )
    }
}

/// Onde a janela de um form estava, para reabrir nela (o `remember_placement` do form).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SavedPlacement {
    /// O `device` do monitor.
    pub monitor: String,
    pub placement: WindowPlacement,
}

/// A posição guardada, se ainda serve: o monitor dela continua ligado e a barra de título
/// fica à vista (pelo menos 120 x 40 pixels dentro da área útil de algum monitor). Senão,
/// `None` (o form abre como se não houvesse posição guardada, no monitor principal), como o
/// Windows faz com a janela de um monitor desligado.
pub fn restore_placement(saved: &SavedPlacement, monitors: &[Monitor]) -> Option<WindowPlacement> {
    let p = saved.placement;
    monitors.iter().find(|m| m.device == saved.monitor)?;
    let visible = monitors.iter().any(|m| {
        let (x, y, w, h) = m.work_area();
        let left = p.x.max(x);
        let right = (p.x + p.width.max(120) as i32).min(x + w as i32);
        let top = p.y.max(y);
        let bottom = (p.y + 40).min(y + h as i32);
        right - left >= 120 && bottom - top >= 40
    });
    visible.then_some(p)
}

impl Ui {
    /// Os monitores, na ordem do sistema (vazio antes de a plataforma informar).
    pub fn monitors(&self) -> &[Monitor] {
        &self.monitors
    }

    /// Conta as mudanças da lista (um monitor ligado, desligado ou com outra resolução): o
    /// `Screen` compara para disparar o `on_change`.
    pub fn monitors_version(&self) -> u64 {
        self.monitors_version
    }

    /// A plataforma informa os monitores (só conta como mudança se a lista mudou).
    pub fn set_monitors(&mut self, monitors: Vec<Monitor>) {
        if monitors != self.monitors {
            self.monitors = monitors;
            self.monitors_version += 1;
        }
    }

    /// O monitor (índice em [`Ui::monitors`]) em que está a janela do form.
    pub fn form_monitor(&self, form: ComponentId) -> Option<usize> {
        self.form_monitors.get(&self.root_of(form)).copied()
    }

    /// A plataforma informa em que monitor está a janela do form.
    pub fn set_form_monitor(&mut self, form: ComponentId, monitor: Option<usize>) {
        match monitor {
            Some(m) => {
                self.form_monitors.insert(form, m);
            }
            None => {
                self.form_monitors.remove(&form);
            }
        }
    }
}
