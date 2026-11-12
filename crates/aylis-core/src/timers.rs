//! Timers (o `TTimer`) e o toque longo.
//!
//! Um componente com [`Component::timer_interval`] recebe o evento `on_timer` a cada
//! intervalo. O `Ui` confere o intervalo quando o componente é criado e quando uma
//! propriedade dele muda (por nome, como no Object Inspector ou no JSON); quem muda os
//! campos direto chama [`Ui::sync_timer`]. A plataforma chama [`Ui::run_timers`] e dorme
//! até [`Ui::next_wake`], que já inclui o próximo timer: parado, nada consome CPU.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::component::ComponentId;
use crate::event::{EventArgs, PointerEvent};
use crate::timing::{self, TimerPrecision};
use crate::ui::Ui;

/// Tempo segurando o dedo (ou o botão) para virar toque longo, em segundos.
pub const LONG_PRESS_TIME: f64 = 0.5;

/// Uma função para rodar uma vez, depois de um tempo ([`Ui::set_timeout`]).
pub(crate) struct Timeout {
    pub(crate) id: u64,
    pub(crate) due: f64,
    pub(crate) job: Box<dyn FnOnce(&mut Ui)>,
}

pub(crate) struct TimerEntry {
    pub(crate) id: ComponentId,
    pub(crate) interval: f64,
    /// Próximo disparo; infinito no timer de thread própria (não entra no despertar da tela).
    pub(crate) next: f64,
    pub(crate) precision: TimerPrecision,
    /// Timer de thread própria: o sinal de parar e a thread (para acordá-la ao parar).
    pub(crate) thread: Option<(Arc<AtomicBool>, std::thread::Thread)>,
}

impl TimerEntry {
    fn start(ui: &Ui, id: ComponentId, interval: f64, precision: TimerPrecision, background: bool) -> Self {
        if precision != TimerPrecision::Normal {
            timing::fine_clock(true);
        }
        let thread = background.then(|| {
            let stop = Arc::new(AtomicBool::new(false));
            let poster = ui.poster();
            let flag = stop.clone();
            let period = Duration::from_secs_f64(interval);
            let spawned = std::thread::Builder::new().name("aylis-timer".into()).spawn(move || {
                let mut next = Instant::now() + period;
                loop {
                    if !timing::park_until(next, precision, &flag) {
                        break;
                    }
                    let now = Instant::now();
                    let late = now.saturating_duration_since(next).as_secs_f64();
                    // A interface acabou (só esta thread ainda tem a fila).
                    if Arc::strong_count(&poster.0) == 1 {
                        break;
                    }
                    let f = flag.clone();
                    poster.post(move |ui| {
                        if !f.load(Ordering::Relaxed) {
                            ui.fire_timer(id, late);
                        }
                    });
                    next += period;
                    if next <= now {
                        next = now + period;
                    }
                }
            });
            match spawned {
                Ok(handle) => (stop, handle.thread().clone()),
                Err(_) => (stop, std::thread::current()),
            }
        });
        let next = if thread.is_some() { f64::INFINITY } else { ui.time + interval };
        Self { id, interval, next, precision, thread }
    }

    /// Para o timer: devolve o relógio fino e encerra a thread.
    pub(crate) fn stop(self) {
        if self.precision != TimerPrecision::Normal {
            timing::fine_clock(false);
        }
        if let Some((flag, thread)) = self.thread {
            flag.store(true, Ordering::Relaxed);
            thread.unpark();
        }
    }
}

impl Ui {
    /// Atualiza o agendamento do timer de `id` (depois de mudar `interval`/`enabled`
    /// direto nos campos do componente).
    pub fn sync_timer(&mut self, id: ComponentId) {
        let wanted = self.component(id).and_then(|c| {
            let interval = c.timer_interval().filter(|i| *i > 0.0)?;
            Some((interval, c.timer_precision(), c.timer_background()))
        });
        let position = self.timers.iter().position(|t| t.id == id);
        match (position, wanted) {
            (Some(i), Some((interval, precision, background))) => {
                let t = &self.timers[i];
                if (t.interval - interval).abs() > f64::EPSILON || t.precision != precision || t.thread.is_some() != background {
                    self.timers.remove(i).stop();
                    let entry = TimerEntry::start(self, id, interval, precision, background);
                    self.timers.push(entry);
                }
            }
            (Some(i), None) => self.timers.remove(i).stop(),
            (None, Some((interval, precision, background))) => {
                let entry = TimerEntry::start(self, id, interval, precision, background);
                self.timers.push(entry);
            }
            (None, None) => {}
        }
    }

    /// Começa a contar de novo o intervalo do timer de `id`, a partir de agora (o próximo
    /// disparo fica a um intervalo inteiro daqui).
    pub fn restart_timer(&mut self, id: ComponentId) {
        if let Some(i) = self.timers.iter().position(|t| t.id == id) {
            self.timers.remove(i).stop();
        }
        self.sync_timer(id);
    }

    /// Dispara o timer de `id` (`late` = quanto atrasou, em segundos).
    fn fire_timer(&mut self, id: ComponentId, late: f64) {
        let _profile = crate::profile::scope(crate::profile::Lane::Timer, || self.path_of(id));
        let _ = self.with_component(id, |c, ctx| {
            c.timer_late(late);
            c.timer(ctx);
        });
        // O componente pode ter parado (ou mudado) o próprio timer.
        self.sync_timer(id);
    }

    /// O próximo disparo de um timer `exact` da tela: a plataforma dorme até um pouco antes
    /// e espera ativa até ele ([`timing::SPIN`]).
    pub fn exact_timer_due(&self) -> Option<f64> {
        self.timers.iter().filter(|t| t.precision == TimerPrecision::Exact && t.thread.is_none()).map(|t| t.next).reduce(f64::min)
    }

    /// Roda `job` uma vez, daqui a `ms` milissegundos (como o `setTimeout` do navegador),
    /// na thread da interface. Devolve o número para cancelar com
    /// [`clear_timeout`](Self::clear_timeout).
    pub fn set_timeout(&mut self, ms: u64, job: impl FnOnce(&mut Ui) + 'static) -> u64 {
        self.next_timeout += 1;
        let id = self.next_timeout;
        self.timeouts.push(Timeout { id, due: self.time + ms as f64 / 1000.0, job: Box::new(job) });
        self.request_repaint();
        id
    }

    /// Cancela um [`set_timeout`](Self::set_timeout) que ainda não rodou.
    pub fn clear_timeout(&mut self, id: u64) {
        self.timeouts.retain(|t| t.id != id);
    }

    /// Dispara os timers vencidos (`on_timer`) e os toques longos. Chamado pela
    /// plataforma a cada volta do loop. Devolve `true` se algo disparou.
    pub fn run_timers(&mut self) -> bool {
        if self.design_mode {
            return false;
        }
        let now = self.time;
        // Os `set_timeout` vencidos, na ordem do vencimento (os criados agora esperam a
        // próxima volta).
        let mut due_now: Vec<(f64, u64)> = self.timeouts.iter().filter(|t| t.due <= now).map(|t| (t.due, t.id)).collect();
        due_now.sort_by(|a, b| a.0.total_cmp(&b.0));
        let ran_timeout = !due_now.is_empty();
        for (_, id) in due_now {
            // Um anterior pode ter cancelado este.
            if let Some(i) = self.timeouts.iter().position(|t| t.id == id) {
                let t = self.timeouts.remove(i);
                (t.job)(self);
            }
        }
        if ran_timeout {
            self.process_events();
        }
        let due: Vec<(ComponentId, f64)> = self.timers.iter().filter(|t| t.next <= now).map(|t| (t.id, now - t.next)).collect();
        for t in self.timers.iter_mut().filter(|t| t.next <= now) {
            // Atrasou (o app ficou ocupado ou suspenso)? Não dispara em rajada.
            t.next = if now - t.next > t.interval { now + t.interval } else { t.next + t.interval };
        }
        for &(id, late) in &due {
            self.fire_timer(id, late);
        }
        let pressed = self.check_long_press();
        let hint = self.check_hints();
        ran_timeout || !due.is_empty() || pressed || hint
    }

    /// Dicas que venceram o tempo de espera: abre (repinta).
    fn check_hints(&mut self) -> bool {
        let now = self.time;
        let delay = self.hint_delay();
        let mut any = false;
        for state in self.input.values_mut() {
            if let Some(h) = state.hint.as_mut().filter(|h| !h.shown && now >= h.since + delay) {
                h.shown = true;
                any = true;
            }
        }
        if any {
            self.request_repaint();
        }
        any
    }

    /// Próximo horário de um timer ou de um toque longo.
    pub(crate) fn timers_wake(&self) -> Option<f64> {
        let timer = self.timers.iter().map(|t| t.next).filter(|n| n.is_finite()).chain(self.timeouts.iter().map(|t| t.due)).reduce(f64::min);
        let delay = self.hint_delay();
        let press = self
            .input
            .values()
            .flat_map(|s| [s.long_press_due(), s.hint_due(delay)])
            .flatten()
            .reduce(f64::min);
        match (timer, press) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }

    /// Toque longo vencido: dispara `on_long_press` e, no toque, abre o menu de contexto.
    fn check_long_press(&mut self) -> bool {
        let now = self.time;
        let roots: Vec<ComponentId> = self.input.keys().copied().collect();
        let mut any = false;
        for root in roots {
            let Some((target, position, touch)) = self.input.get_mut(&root).and_then(|s| s.take_long_press(now)) else {
                continue;
            };
            // Só o dedo (ou a caneta): o mouse parado com o botão apertado não é toque longo
            // (o relatório do Tabernáculo, J18).
            if !touch || !self.contains(target) {
                continue;
            }
            any = true;
            let local = self.local_point(target, position);
            let event = PointerEvent { position: local, kind: crate::event::PointerKind::Touch, ..Default::default() };
            self.fire(target, "on_long_press", &EventArgs::Pointer(event));
            self.open_popup_menu(target, position);
            self.request_repaint();
        }
        any
    }
}
