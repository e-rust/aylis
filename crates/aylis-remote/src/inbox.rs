//! A fila do que chega do backend até a thread da tela (o `aylis-host`).
//!
//! Duas garantias, para um backend que manda mais rápido do que a tela trata (um terminal de
//! cotações, um sensor):
//!
//! - **a fila tem limite:** cheia, quem lê o backend espera ([`InboxSender::send`] bloqueia), e
//!   a espera volta ao backend pelo próprio canal (o pipe do stdio ou o TCP do WebSocket
//!   enchem). Nada se perde e a memória não cresce;
//! - **a tela continua pintando:** cada rodada trata só um orçamento de tempo
//!   ([`Inbox::drain`]) e devolve [`Drain::More`]; quem chama pinta e acorda de novo. As
//!   mudanças que chegaram entre dois quadros saem juntas num quadro só.
//!
//! Um acordar por rodada: o remetente só acorda a tela se ela ainda não foi acordada (sem um
//! evento do sistema por mensagem, que também disputariam com a pintura).

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError};
use std::time::Duration;

use aylis_core::Ui;
use serde_json::Value;
use web_time::Instant;

/// Quantas mensagens cabem na fila (um pouco mais que um quadro a 100 mil por segundo).
pub const INBOX_CAPACITY: usize = 4096;

/// Quanto tempo uma rodada trata mensagens antes de deixar a tela pintar: metade de um
/// quadro a 60 Hz.
pub const INBOX_BUDGET: Duration = Duration::from_millis(8);

/// Cria a fila: o remetente vai para a thread que lê o backend; a fila fica na da tela.
pub fn inbox(capacity: usize) -> (InboxSender, Inbox) {
    let (tx, rx) = mpsc::sync_channel(capacity.max(1));
    let awake = Arc::new(AtomicBool::new(false));
    (InboxSender { tx, awake: awake.clone() }, Inbox { rx, awake, longest: 0, total: 0 })
}

/// O lado de quem lê o backend.
pub struct InboxSender {
    tx: SyncSender<Value>,
    awake: Arc<AtomicBool>,
}

impl InboxSender {
    /// Põe a mensagem na fila (esperando, se estiver cheia) e acorda a tela se preciso.
    /// `Err` quando a tela já fechou.
    pub fn send(&self, message: Value, wake: impl Fn()) -> Result<(), ()> {
        self.tx.send(message).map_err(|_| ())?;
        if !self.awake.swap(true, Ordering::AcqRel) {
            wake();
        }
        Ok(())
    }
}

/// O resultado de uma rodada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Drain {
    /// A fila esvaziou.
    Empty,
    /// O orçamento acabou com mensagens na fila: pinte e chame de novo.
    More,
    /// O backend terminou e a fila esvaziou.
    Closed,
}

/// O lado da tela.
pub struct Inbox {
    rx: Receiver<Value>,
    awake: Arc<AtomicBool>,
    longest: usize,
    total: usize,
}

impl Inbox {
    /// Trata as mensagens até a fila esvaziar ou o `budget` acabar; cada resposta vai para
    /// `reply`. Com [`Drain::More`], quem chama acorda a tela de novo depois de pintar.
    pub fn drain(&mut self, ui: &mut Ui, budget: Duration, mut reply: impl FnMut(Value)) -> Drain {
        // Antes de ler: o que chegar daqui em diante acorda de novo.
        self.awake.store(false, Ordering::Release);
        let started = Instant::now();
        let mut handled = 0usize;
        loop {
            match self.rx.try_recv() {
                Ok(message) => {
                    handled += 1;
                    self.total += 1;
                    if let Some(answer) = crate::handle_message(ui, &message) {
                        reply(answer);
                    }
                    // O relógio só a cada 16 mensagens (uma rodada nunca para antes de 16).
                    if handled % 16 == 0 && started.elapsed() >= budget {
                        self.longest = self.longest.max(handled);
                        self.awake.store(true, Ordering::Release);
                        return Drain::More;
                    }
                }
                Err(TryRecvError::Empty) => {
                    self.longest = self.longest.max(handled);
                    return Drain::Empty;
                }
                Err(TryRecvError::Disconnected) => return Drain::Closed,
            }
        }
    }

    /// O maior número de mensagens tratado numa rodada (para as medições).
    pub fn longest_round(&self) -> usize {
        self.longest
    }

    /// Quantas mensagens foram tratadas desde o início.
    pub fn handled(&self) -> usize {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::atomic::AtomicUsize;

    fn ui() -> Ui {
        let mut ui = Ui::new();
        crate::register_all(&mut ui);
        crate::execute(&mut ui, &json!({ "cmd": "load", "layout": { "root": { "class": "Form", "name": "frm", "children": [{ "class": "Label", "name": "lbl" }] } } })).unwrap();
        ui
    }

    /// A fila cheia faz o remetente esperar (em vez de crescer), e a tela recebe tudo, em
    /// ordem.
    #[test]
    fn a_full_inbox_makes_the_sender_wait() {
        let (tx, mut inbox) = inbox(8);
        let wakes = Arc::new(AtomicUsize::new(0));
        let w = wakes.clone();
        let producer = std::thread::spawn(move || {
            for i in 0..1000 {
                tx.send(json!({ "cmd": "set", "target": "lbl", "prop": "text", "value": format!("{i}") }), || {
                    w.fetch_add(1, Ordering::Relaxed);
                })
                .unwrap();
            }
        });
        let mut ui = ui();
        let mut rounds = 0;
        loop {
            rounds += 1;
            let result = inbox.drain(&mut ui, INBOX_BUDGET, |_| {});
            if result == Drain::Closed {
                break;
            }
            if rounds % 4 == 0 {
                std::thread::yield_now();
            }
        }
        producer.join().unwrap();
        assert!(inbox.longest_round() <= 1000);
        let lbl = ui.find("lbl").unwrap();
        assert_eq!(ui.get_prop(lbl, "text").unwrap(), aylis_core::PropValue::String("999".into()));
        // Um acordar por rodada, não por mensagem.
        assert!(wakes.load(Ordering::Relaxed) < 1000, "{} acordares", wakes.load(Ordering::Relaxed));
    }

    /// Com o orçamento zerado, cada rodada para cedo e diz que há mais.
    #[test]
    fn the_budget_stops_the_round() {
        let (tx, mut inbox) = inbox(INBOX_CAPACITY);
        for i in 0..100 {
            tx.send(json!({ "cmd": "set", "target": "lbl", "prop": "text", "value": format!("{i}") }), || {}).unwrap();
        }
        let mut ui = ui();
        assert_eq!(inbox.drain(&mut ui, Duration::ZERO, |_| {}), Drain::More);
        assert_eq!(inbox.longest_round(), 16);
        assert_eq!(inbox.drain(&mut ui, INBOX_BUDGET, |_| {}), Drain::Empty);
        drop(tx);
        assert_eq!(inbox.drain(&mut ui, INBOX_BUDGET, |_| {}), Drain::Closed);
    }
}
