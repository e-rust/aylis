//! O teclado fora dos eventos de tecla: as travas agora ([`Ui::lock_keys`]) e a captura das
//! teclas do sistema (o componente `KeyboardCapture`, decidido em 24/11).
//!
//! A captura é pedida por componente ([`Ui::set_keyboard_capture`]) e quem prende o teclado de
//! verdade é a plataforma: ela pergunta, para a janela em foco, se o form dela quer
//! ([`Ui::keyboard_capture`]), prende só enquanto a janela está em primeiro plano e avisa o
//! que fez ([`Ui::set_keyboard_captured`]).

use crate::component::ComponentId;
use crate::event::EventArgs;
use crate::prop::PropValue;
use crate::ui::Ui;

/// As travas do teclado ligadas agora.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LockKeys {
    pub caps_lock: bool,
    pub num_lock: bool,
    pub scroll_lock: bool,
}

/// O registro das travas no acesso tipado (o `Screen.lock_keys`).
pub const LOCK_KEYS_RECORD: crate::prop::RecordInfo = crate::prop::RecordInfo {
    name: "LockKeys",
    fields: &[("caps_lock", crate::PropKind::Bool), ("num_lock", crate::PropKind::Bool), ("scroll_lock", crate::PropKind::Bool)],
};

impl LockKeys {
    /// Em JSON (o `Screen.lock_keys`).
    pub fn to_json(&self) -> String {
        format!("{{\"caps_lock\":{},\"num_lock\":{},\"scroll_lock\":{}}}", self.caps_lock, self.num_lock, self.scroll_lock)
    }
}

/// O que um `KeyboardCapture` ativo pede.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyboardCaptureRequest {
    /// O Alt+F4 também vai para o app (sem isso, fecha a janela, como sempre).
    pub alt_f4: bool,
}

/// A plataforma sabe prender o teclado (Windows, X11 e Wayland).
static CAPTURE_AVAILABLE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// A plataforma diz que prende o teclado (o `available` do `KeyboardCapture`).
pub fn set_capture_available(available: bool) {
    CAPTURE_AVAILABLE.store(available, std::sync::atomic::Ordering::Relaxed);
}

/// A captura das teclas do sistema existe nesta plataforma.
pub fn capture_available() -> bool {
    CAPTURE_AVAILABLE.load(std::sync::atomic::Ordering::Relaxed)
}

/// Os pedidos de captura e o form que a plataforma prendeu.
#[derive(Default)]
struct CaptureState {
    requests: Vec<(ComponentId, KeyboardCaptureRequest)>,
    captured: Option<ComponentId>,
}

impl Ui {
    /// O Caps Lock, o Num Lock e o Scroll Lock agora, fora de um evento de tecla (o
    /// `Modifiers` de cada tecla e do mouse traz os mesmos três). No Windows, o `GetKeyState`;
    /// no X11 (e no Wayland com o XWayland), o servidor X; no macOS, o Caps Lock (o Mac não tem
    /// Num Lock nem Scroll Lock); no navegador, o que a última tecla disse; no celular e sem
    /// plataforma, tudo desligado.
    pub fn lock_keys(&self) -> LockKeys {
        self.services().desktop.as_ref().and_then(|d| d.lock_keys()).unwrap_or_default()
    }

    /// O componente `id` pede (`Some`) ou deixa de pedir (`None`) a captura das teclas do
    /// sistema para o form dele. Quem usa é o `KeyboardCapture`; a plataforma prende o teclado
    /// só com a janela do form em primeiro plano.
    pub fn set_keyboard_capture(&mut self, id: ComponentId, request: Option<KeyboardCaptureRequest>) {
        // Os pedidos de componentes já liberados saem aqui.
        let mut requests = std::mem::take(&mut self.extension::<CaptureState>().requests);
        requests.retain(|(c, _)| *c != id && self.contains(*c));
        if let Some(request) = request {
            requests.push((id, request));
        }
        self.extension::<CaptureState>().requests = requests;
    }

    /// O que os componentes do form `form` pedem (`None`: nenhum pede). Com mais de um, o
    /// Alt+F4 vai se um deles pedir.
    pub fn keyboard_capture(&self, form: ComponentId) -> Option<KeyboardCaptureRequest> {
        let state = self.extension_ref::<CaptureState>()?;
        let mut out: Option<KeyboardCaptureRequest> = None;
        for (id, request) in &state.requests {
            if self.contains(*id) && self.root_of(*id) == form {
                let merged = out.get_or_insert_with(KeyboardCaptureRequest::default);
                merged.alt_f4 |= request.alt_f4;
            }
        }
        out
    }

    /// A plataforma avisa que prendeu o teclado na janela de `form` (ou soltou, com `None`):
    /// os `KeyboardCapture` dos forms que mudaram recebem o `on_capture_change`.
    pub fn set_keyboard_captured(&mut self, form: Option<ComponentId>) {
        let state = self.extension::<CaptureState>();
        let old = std::mem::replace(&mut state.captured, form);
        if old == form {
            return;
        }
        let ids: Vec<ComponentId> = state.requests.iter().map(|(id, _)| *id).collect();
        for id in ids {
            if !self.contains(id) {
                continue;
            }
            let root = self.root_of(id);
            if Some(root) == old || Some(root) == form {
                self.fire(id, "on_capture_change", &EventArgs::Value(PropValue::Bool(Some(root) == form)));
            }
        }
    }

    /// O form cuja janela está com o teclado preso agora (`None`: nenhum).
    pub fn keyboard_captured(&self) -> Option<ComponentId> {
        self.extension_ref::<CaptureState>().and_then(|s| s.captured)
    }
}
