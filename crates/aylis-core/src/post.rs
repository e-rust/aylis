//! Rodar código na thread da interface a partir de outra thread.
//!
//! ```
//! # use aylis_core::Ui;
//! let mut ui = Ui::new();
//! let poster = ui.poster(); // pode ir para outra thread
//! std::thread::spawn(move || {
//!     poster.post(|ui| ui.request_repaint());
//! })
//! .join()
//! .unwrap();
//! assert_eq!(ui.process_posted(), 1);
//! ```
//!
//! A plataforma instala a função que acorda a interface ([`Ui::set_wake`]) e chama
//! [`Ui::process_posted`] quando acorda.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::ui::Ui;

/// Uma função para rodar na thread da interface.
pub type Job = Box<dyn FnOnce(&mut Ui) + Send>;

/// Acorda a interface (vem da plataforma; pode ser chamada de qualquer thread).
pub type WakeFn = Arc<dyn Fn() + Send + Sync>;

#[derive(Default)]
pub(crate) struct PostQueue {
    jobs: Mutex<VecDeque<Job>>,
    wake: Mutex<Option<WakeFn>>,
}

/// Entrega funções para a thread da interface. Barato de clonar; vale de qualquer thread.
#[derive(Clone)]
pub struct Poster(pub(crate) Arc<PostQueue>);

impl Poster {
    /// Agenda `job` para rodar na thread da interface e acorda a interface.
    pub fn post(&self, job: impl FnOnce(&mut Ui) + Send + 'static) {
        if let Ok(mut jobs) = self.0.jobs.lock() {
            jobs.push_back(Box::new(job));
        }
        self.wake();
    }

    /// Só acorda a interface (ela repinta se algo mudou, e roda o que estiver agendado).
    pub fn wake(&self) {
        let wake = self.0.wake.lock().ok().and_then(|w| w.clone());
        if let Some(wake) = wake {
            wake();
        }
    }
}

impl Ui {
    /// Um [`Poster`] para esta interface (para threads de trabalho, áudio, vídeo, rede).
    pub fn poster(&self) -> Poster {
        Poster(self.posted.clone())
    }

    /// A plataforma diz como acordar a interface.
    pub fn set_wake(&mut self, wake: WakeFn) {
        if let Ok(mut w) = self.posted.wake.lock() {
            *w = Some(wake);
        }
    }

    /// Roda o que outras threads agendaram. Devolve quantas funções rodaram.
    pub fn process_posted(&mut self) -> usize {
        let mut count = 0;
        loop {
            let job = self.posted.jobs.lock().ok().and_then(|mut jobs| jobs.pop_front());
            let Some(job) = job else { break };
            job(self);
            count += 1;
        }
        // Também os eventos que sobraram da volta anterior (veja `process_events`).
        if count > 0 || self.has_pending_events() {
            self.process_events();
        }
        count
    }
}
