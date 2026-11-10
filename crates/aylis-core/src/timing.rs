//! A precisão dos timers ([`TimerPrecision`]) e o esperar até um instante com ela.
//!
//! O núcleo não chama o sistema: a plataforma registra como pedir ao sistema o relógio de 1 ms
//! ([`set_resolution_hook`]; no Windows, o `timeBeginPeriod`, que sem pedido acorda em degraus
//! de ~15,6 ms). Os pedidos são contados: enquanto um timer `high` ou `exact` estiver ligado, o
//! relógio fino fica pedido; o último a desligar devolve.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Quanto um timer pode atrasar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimerPrecision {
    /// O relógio normal do sistema (no Windows, degraus de ~15,6 ms): o que menos gasta CPU e
    /// bateria. O padrão.
    #[default]
    Normal,
    /// ~1 ms: pede ao sistema o relógio fino enquanto o timer estiver ligado.
    High,
    /// Abaixo de 0,1 ms: dorme até ~1,5 ms antes e termina esperando ativo (um núcleo ocupado
    /// nesse trecho).
    Exact,
}

impl crate::prop::PropEnum for TimerPrecision {
    const NAMES: &'static [&'static str] = &["normal", "high", "exact"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Normal, Self::High, Self::Exact].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Quanto antes do prazo o `exact` para de dormir e passa a esperar ativo.
pub const SPIN: Duration = Duration::from_micros(1500);

struct Resolution {
    hook: Option<fn(bool)>,
    requests: usize,
}

static RESOLUTION: Mutex<Resolution> = Mutex::new(Resolution { hook: None, requests: 0 });

/// A plataforma registra como ligar (`true`) e desligar o relógio fino do sistema.
pub fn set_resolution_hook(hook: fn(bool)) {
    if let Ok(mut r) = RESOLUTION.lock() {
        r.hook = Some(hook);
        // Timers criados antes de a plataforma começar.
        if r.requests > 0 {
            hook(true);
        }
    }
}

/// Pede (`true`) ou devolve o relógio fino; os pedidos são contados.
pub fn fine_clock(on: bool) {
    let Ok(mut r) = RESOLUTION.lock() else { return };
    let before = r.requests;
    r.requests = if on { before + 1 } else { before.saturating_sub(1) };
    if let Some(hook) = r.hook
        && (before == 0) != (r.requests == 0)
    {
        hook(r.requests > 0);
    }
}

/// Espera até `deadline` com a precisão pedida, numa thread que não é a da tela (o `exact`
/// ocupa a CPU no fim). Acorda antes se `stop` ligar (quem liga chama `unpark` na thread).
/// Devolve `false` se parou.
pub fn park_until(deadline: Instant, precision: TimerPrecision, stop: &AtomicBool) -> bool {
    let margin = if precision == TimerPrecision::Exact { SPIN } else { Duration::ZERO };
    loop {
        if stop.load(Ordering::Relaxed) {
            return false;
        }
        let now = Instant::now();
        if now + margin >= deadline {
            break;
        }
        std::thread::park_timeout(deadline - margin - now);
    }
    while Instant::now() < deadline {
        std::hint::spin_loop();
    }
    !stop.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    static CALLS: AtomicUsize = AtomicUsize::new(0);

    fn hook(on: bool) {
        CALLS.fetch_add(if on { 1 } else { 100 }, Ordering::SeqCst);
    }

    #[test]
    fn requests_are_counted_and_exact_is_exact() {
        set_resolution_hook(hook);
        fine_clock(true);
        fine_clock(true);
        fine_clock(false);
        assert_eq!(CALLS.load(Ordering::SeqCst), 1, "só o primeiro pedido liga");
        fine_clock(false);
        assert_eq!(CALLS.load(Ordering::SeqCst), 101, "o último devolve");
        // Dentro do `SPIN` (o trecho de dormir depende do relógio fino, que aqui é de mentira).
        let deadline = Instant::now() + Duration::from_millis(1);
        assert!(park_until(deadline, TimerPrecision::Exact, &AtomicBool::new(false)));
        let late = Instant::now() - deadline;
        assert!(late < Duration::from_millis(1), "atrasou {late:?}");
    }
}
