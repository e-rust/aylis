//! O serviço em segundo plano (o componente `BackgroundService`): a lógica sem tela do app (a
//! unidade `background` do projeto, `pub fn run(service: Service)`) numa thread que continua
//! com a tela fechada. No Android ela fica num serviço em primeiro plano (a notificação fixa)
//! e o sistema não mata o processo; no desktop, enquanto o app roda.
//!
//! A tela e o serviço conversam por mensagens de texto: [`send_to_service`] (o `send` do
//! componente) e [`Service::send`] (que chega no `on_message` dele). As mensagens esperam
//! numa fila: a tela fechada e aberta de novo recebe o que o serviço mandou nesse meio tempo.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

/// A lógica sem tela do app.
pub type Worker = fn(Service);

struct Queues {
    to_service: VecDeque<String>,
    to_ui: VecDeque<String>,
    running: bool,
    stop: bool,
}

static QUEUES: Mutex<Queues> = Mutex::new(Queues { to_service: VecDeque::new(), to_ui: VecDeque::new(), running: false, stop: false });
static WAKE: Condvar = Condvar::new();
static WORKER: Mutex<Option<Worker>> = Mutex::new(None);

/// O lado do serviço: recebe as mensagens da tela e manda as dele.
pub struct Service {
    _private: (),
}

impl Service {
    /// A próxima mensagem da tela, esperando até `timeout` (`None`: nada chegou).
    pub fn recv(&self, timeout: Duration) -> Option<String> {
        let mut q = QUEUES.lock().ok()?;
        if q.to_service.is_empty() && !q.stop {
            q = WAKE.wait_timeout(q, timeout).ok()?.0;
        }
        q.to_service.pop_front()
    }

    /// Manda uma mensagem à tela (o `on_message` do `BackgroundService`).
    pub fn send(&self, text: impl Into<String>) {
        if let Ok(mut q) = QUEUES.lock() {
            q.to_ui.push_back(text.into());
        }
    }

    /// O serviço foi parado (`stop` do componente): a lógica deve terminar.
    pub fn stopped(&self) -> bool {
        QUEUES.lock().is_ok_and(|q| q.stop)
    }
}

/// O app diz qual é a lógica sem tela (a geração faz isso no `app()`).
pub fn set_worker(worker: Worker) {
    if let Ok(mut w) = WORKER.lock() {
        *w = Some(worker);
    }
}

/// Começa a lógica (uma vez por processo; de novo só depois de ela terminar). `worker`: a do
/// app; `None`, a registrada por [`set_worker`]. Devolve se havia lógica para rodar.
pub fn start(worker: Option<Worker>) -> bool {
    let Some(worker) = worker.or_else(|| WORKER.lock().ok().and_then(|w| *w)) else { return false };
    {
        let Ok(mut q) = QUEUES.lock() else { return false };
        q.stop = false;
        if q.running {
            return true;
        }
        q.running = true;
    }
    std::thread::Builder::new()
        .name("aylis-background".into())
        .spawn(move || {
            worker(Service { _private: () });
            if let Ok(mut q) = QUEUES.lock() {
                q.running = false;
            }
        })
        .is_ok()
}

/// Pede à lógica que termine (ela vê em [`Service::stopped`] e acorda do `recv`).
pub fn stop() {
    if let Ok(mut q) = QUEUES.lock() {
        q.stop = true;
    }
    WAKE.notify_all();
}

/// A lógica está rodando.
pub fn running() -> bool {
    QUEUES.lock().is_ok_and(|q| q.running)
}

/// Manda uma mensagem ao serviço (o `send` do componente).
pub fn send_to_service(text: impl Into<String>) {
    if let Ok(mut q) = QUEUES.lock() {
        q.to_service.push_back(text.into());
    }
    WAKE.notify_all();
}

/// Tira da fila o que o serviço mandou para a tela.
pub fn take_messages() -> Vec<String> {
    QUEUES.lock().map(|mut q| q.to_ui.drain(..).collect()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn echo(service: Service) {
        while !service.stopped() {
            if let Some(m) = service.recv(Duration::from_millis(50)) {
                service.send(format!("eco: {m}"));
            }
        }
    }

    #[test]
    fn the_service_runs_once_and_talks_with_the_screen() {
        assert!(!start(None) || running(), "sem lógica registrada, não começa");
        set_worker(echo);
        assert!(start(None) && start(None), "a segunda vez não abre outra thread");
        send_to_service("oi");
        let mut got = Vec::new();
        for _ in 0..100 {
            got.extend(take_messages());
            if !got.is_empty() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(got, ["eco: oi"]);
        stop();
        for _ in 0..100 {
            if !running() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(!running());
    }
}
