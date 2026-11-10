//! Observadores de atividade: tudo o que o usuário faz no app (ponteiro, roda, teclas,
//! texto) e o que o app responde (eventos com handler, forms que abrem e fecham, erros)
//! chega a quem estiver ouvindo. É a base do `EventRecorder` (grava a sessão num arquivo
//! que a IDE abre) e do `Analytics` (métricas de uso), sem o núcleo saber de arquivos.
//!
//! A entrada é avisada antes de ser tratada (mesmo se um modal a bloquear: o clique na
//! janela de baixo também conta), com as coordenadas do form.

use crate::component::ComponentId;
use crate::event::{KeyEvent, PointerEvent, WheelEvent};
use crate::ui::Ui;

/// Algo que aconteceu no app.
#[derive(Clone, Debug)]
pub enum Activity<'a> {
    PointerDown { form: ComponentId, event: PointerEvent },
    PointerUp { form: ComponentId, event: PointerEvent },
    /// O ponteiro andou (muitos por segundo: quem grava decide se guarda).
    PointerMove { form: ComponentId, event: PointerEvent },
    Wheel { form: ComponentId, event: WheelEvent },
    KeyDown { form: ComponentId, event: KeyEvent },
    KeyUp { form: ComponentId, event: KeyEvent },
    Text { form: ComponentId, text: &'a str },
    /// Um evento com handler disparou (`handler` vazio: closure em Rust).
    Event { component: ComponentId, event: &'a str, handler: &'a str },
    FormShown { form: ComponentId, modal: bool },
    FormClosed { form: ComponentId },
    /// Um erro do app (pânico, falha que o código quis registrar).
    Error { message: &'a str },
    /// Uma métrica do próprio app (`Analytics::track`): nome e valor.
    Custom { name: &'a str, value: &'a str },
}

/// Quem ouve: recebe o `Ui` (para achar caminhos, classes, o controle sob o ponteiro).
pub type ObserverFn = dyn FnMut(&Ui, &Activity<'_>);

/// O registro de um observador, para tirá-lo depois.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObserverId(pub(crate) u64);

#[derive(Default)]
pub(crate) struct Observers {
    next: u64,
    list: Vec<(ObserverId, Box<ObserverFn>)>,
    /// Durante um aviso a lista sai daqui; tirar um observador nessa hora fica anotado.
    removed: Vec<ObserverId>,
    /// O que a plataforma conta de si (renderizador, placa de vídeo...), para as gravações.
    info: Vec<(String, String)>,
}

impl Ui {
    /// Passa a avisar `f` de toda a atividade do app.
    pub fn add_observer(&mut self, f: impl FnMut(&Ui, &Activity<'_>) + 'static) -> ObserverId {
        let obs = self.observers_mut();
        obs.next += 1;
        let id = ObserverId(obs.next);
        obs.list.push((id, Box::new(f)));
        id
    }

    pub fn remove_observer(&mut self, id: ObserverId) {
        let obs = self.observers_mut();
        obs.list.retain(|(i, _)| *i != id);
        obs.removed.push(id);
    }

    /// A plataforma conta algo de si (`"renderizador"`, `"gpu"`...): as gravações guardam
    /// no cabeçalho, e a mudança também vai para quem ouve (como métrica).
    pub fn set_platform_info(&mut self, key: &str, value: &str) {
        let info = &mut self.observers_mut().info;
        match info.iter_mut().find(|(k, _)| k == key) {
            Some((_, v)) if v == value => return,
            Some((_, v)) => *v = value.to_string(),
            None => info.push((key.to_string(), value.to_string())),
        }
        self.report(&Activity::Custom { name: key, value });
    }

    pub fn platform_info(&self) -> &[(String, String)] {
        &self.observers().info
    }

    /// Há alguém ouvindo? (para não montar a atividade à toa).
    pub fn has_observers(&self) -> bool {
        !self.observers().list.is_empty()
    }

    /// Avisa os observadores. O app também usa para erros e métricas próprias.
    pub fn report(&mut self, activity: &Activity<'_>) {
        if !self.has_observers() {
            return;
        }
        let mut list = std::mem::take(&mut self.observers_mut().list);
        for (_, f) in &mut list {
            f(self, activity);
        }
        let obs = self.observers_mut();
        // Os que entraram durante o aviso ficam depois; os tirados durante, saem.
        list.append(&mut obs.list);
        let removed = std::mem::take(&mut obs.removed);
        list.retain(|(i, _)| !removed.contains(i));
        obs.list = list;
    }
}
