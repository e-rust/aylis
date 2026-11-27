//! O que a janela de um form informa e o app lê: a escala real dela (o `on_scale_change`), o
//! fim de um redimensionamento (o `on_resize_end`), o mouse sobre ela e o token de ativação
//! para trazê-la para a frente no Wayland. A plataforma informa; o `Ui` guarda e dispara os
//! eventos do form.

use std::collections::HashMap;

use crate::component::ComponentId;
use crate::event::EventArgs;
use crate::prop::PropValue;
use crate::ui::Ui;

/// Quanto tempo sem um tamanho novo encerra um redimensionamento onde o sistema não avisa o
/// fim (fora do Windows, e no Windows fora do arrasto da borda: maximizar, encaixar).
pub const RESIZE_END_DELAY_MS: u64 = 300;

#[derive(Default)]
struct FormWindows {
    /// A escala de cada janela, a última informada.
    scales: HashMap<ComponentId, f32>,
    /// O último tamanho (pixels físicos) de cada janela, para só contar mudança de verdade.
    sizes: HashMap<ComponentId, (u32, u32)>,
    /// Os redimensionamentos em andamento: o prazo do fim (`None`: o usuário ainda segura a
    /// borda, e o sistema avisa quando soltar).
    resizing: HashMap<ComponentId, Option<u64>>,
    /// O token de ativação dado pelo app (o `XDG_ACTIVATION_TOKEN` de outra abertura).
    activation_token: Option<String>,
    /// As janelas que já informaram o monitor (o `on_monitor_change` não sai na primeira).
    monitor_reported: std::collections::HashSet<ComponentId>,
}

impl Ui {
    /// A escala da janela do form agora: quantos pixels físicos há numa unidade sem zoom (1.0
    /// = 96 dpi no Windows, 2.0 numa tela Retina). É a da janela, que no Wayland com escala
    /// fracionária pode não ser a do monitor; com `physical_pixels` ligado, continua a do
    /// sistema (o form é que desenha em 1:1). Antes de a janela existir (no `on_show` ela pode
    /// ainda não ter sido criada: leia no `on_window_created`) e sem janela, nos testes, 1.0. A
    /// mudança chega pelo `on_scale_change` do form, com a escala nova.
    pub fn form_scale(&self, form: ComponentId) -> f32 {
        let form = self.root_of(form);
        self.extension_ref::<FormWindows>().and_then(|w| w.scales.get(&form).copied()).unwrap_or(1.0)
    }

    /// A plataforma informa a escala da janela do form (ao abrir e a cada mudança). Se mudou
    /// (não na primeira vez), dispara o `on_scale_change` do form com a escala nova.
    pub fn set_form_scale(&mut self, form: ComponentId, scale: f32) {
        if !(scale.is_finite() && scale > 0.0) {
            return;
        }
        let old = self.extension::<FormWindows>().scales.insert(form, scale);
        if old.is_some_and(|o| (o - scale).abs() > f32::EPSILON) {
            self.fire(form, "on_scale_change", &EventArgs::Value(PropValue::Float(f64::from(scale))));
        }
    }

    /// Anota que a janela do form informou o monitor: `true` na primeira vez desde que abriu
    /// (o monitor de abrir, ou nenhum, no Wayland).
    pub(crate) fn note_form_monitor_reported(&mut self, form: ComponentId) -> bool {
        self.extension::<FormWindows>().monitor_reported.insert(form)
    }

    /// A plataforma informa o tamanho novo da janela do form (pixels físicos). `holding`: o
    /// usuário ainda segura a borda e o sistema avisa quando soltar
    /// ([`form_resize_released`](Self::form_resize_released), o `WM_EXITSIZEMOVE` do Windows);
    /// sem isso, o fim é [`RESIZE_END_DELAY_MS`] sem tamanho novo. O mesmo tamanho de antes e a
    /// janela minimizada (0 × 0) não contam.
    pub fn form_resized(&mut self, form: ComponentId, width: u32, height: u32, holding: bool) {
        if width == 0 || height == 0 {
            return;
        }
        let state = self.extension::<FormWindows>();
        let old = state.sizes.insert(form, (width, height));
        if old.is_none_or(|o| o == (width, height)) {
            // A primeira vez é o tamanho de abrir, não um redimensionamento.
            return;
        }
        if let Some(Some(timeout)) = state.resizing.remove(&form) {
            self.clear_timeout(timeout);
        }
        let due = (!holding).then(|| {
            self.set_timeout(RESIZE_END_DELAY_MS, move |ui| {
                if ui.extension::<FormWindows>().resizing.remove(&form).is_some() {
                    ui.fire_resize_end(form);
                }
            })
        });
        self.extension::<FormWindows>().resizing.insert(form, due);
    }

    /// A plataforma informa que o usuário soltou a janela do form depois de arrastar ou
    /// redimensionar: se o tamanho mudou, o `on_resize_end` sai já.
    pub fn form_resize_released(&mut self, form: ComponentId) {
        let Some(pending) = self.extension::<FormWindows>().resizing.remove(&form) else { return };
        if let Some(timeout) = pending {
            self.clear_timeout(timeout);
        }
        self.fire_resize_end(form);
    }

    /// A janela do form está sendo redimensionada (o `on_resize_end` ainda não saiu).
    pub fn is_resizing(&self, form: ComponentId) -> bool {
        let form = self.root_of(form);
        self.extension_ref::<FormWindows>().is_some_and(|w| w.resizing.contains_key(&form))
    }

    fn fire_resize_end(&mut self, form: ComponentId) {
        if self.is_open(form) {
            self.fire(form, "on_resize_end", &EventArgs::None);
        }
    }

    /// A plataforma esquece a janela do form que fechou (a escala e o tamanho voltam a valer
    /// como da primeira vez ao abrir de novo).
    pub fn forget_form_window(&mut self, form: ComponentId) {
        let state = self.extension::<FormWindows>();
        state.scales.remove(&form);
        state.sizes.remove(&form);
        state.monitor_reported.remove(&form);
        if let Some(Some(timeout)) = state.resizing.remove(&form) {
            self.clear_timeout(timeout);
        }
    }

    /// O mouse está sobre a janela do form (ou do form de um controle) à vista, sem outra
    /// janela por cima: o [`cursor_position`](Self::cursor_position) e o
    /// [`form_at_screen`](Self::form_at_screen) juntos. Sempre `false` onde o sistema não diz
    /// onde está o mouse fora de um evento (Wayland, celular e navegador): lá, os eventos de
    /// mouse do próprio form (`on_pointer_enter`/`on_pointer_leave`) dizem.
    pub fn pointer_over(&self, form: ComponentId) -> bool {
        let form = self.root_of(form);
        self.cursor_position().and_then(|c| self.form_at_screen(c.x, c.y)) == Some(form)
    }

    /// Guarda o token de ativação que o próximo [`WindowCommand::BringToFront`](crate::WindowCommand::BringToFront)
    /// usa no Wayland, uma vez (vazio: nenhum). O token vem de quem abriu o app: o
    /// `XDG_ACTIVATION_TOKEN` da outra abertura, passado pelo app (o do próprio processo a
    /// plataforma já lê ao abrir). Fora do Wayland, sem efeito.
    pub fn set_activation_token(&mut self, token: &str) {
        let token = token.trim();
        self.extension::<FormWindows>().activation_token = (!token.is_empty()).then(|| token.to_string());
    }

    /// A plataforma pega o token de ativação guardado (uma vez: o compositor só aceita cada
    /// token uma vez).
    pub fn take_activation_token(&mut self) -> Option<String> {
        self.extension::<FormWindows>().activation_token.take()
    }
}
