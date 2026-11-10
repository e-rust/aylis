//! Profiler (como o QML Profiler): blocos com começo e duração em faixas (handlers, eventos,
//! layout, pintura, timers, animações, dados, recursos e as marcas à mão). Desligado, cada
//! ponto de medição custa uma leitura atômica.
//!
//! O app de desenvolvimento (`dev`) liga pelo canal da IDE (`{"cmd": "profile", "on": true}`)
//! e a IDE busca os blocos de tempos em tempos (`profile_take`). No código:
//!
//! ```
//! let _p = aylis_core::profile::profile("carregar clientes");
//! // ... o que demora ...
//! ```

use std::cell::Cell;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// A faixa de um bloco na linha do tempo.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Lane {
    /// Um handler (o form e o método).
    Handler,
    /// Um evento sem handler nomeado (closure no Rust).
    Event,
    Layout,
    Paint,
    Timer,
    Animation,
    Data,
    Resource,
    /// Marcado à mão ([`profile`]).
    Mark,
    /// A GPU no quadro: o tempo de mandar e apresentar o quadro, fora a pintura (a plataforma
    /// mede).
    Gpu,
}

impl Lane {
    pub const ALL: [Lane; 10] = [Lane::Handler, Lane::Event, Lane::Layout, Lane::Paint, Lane::Gpu, Lane::Timer, Lane::Animation, Lane::Data, Lane::Resource, Lane::Mark];

    /// O nome no formato de troca (`handler`, `layout`...).
    pub fn name(self) -> &'static str {
        match self {
            Lane::Handler => "handler",
            Lane::Event => "event",
            Lane::Layout => "layout",
            Lane::Paint => "paint",
            Lane::Timer => "timer",
            Lane::Animation => "animation",
            Lane::Data => "data",
            Lane::Resource => "resource",
            Lane::Mark => "mark",
            Lane::Gpu => "gpu",
        }
    }

    pub fn parse(name: &str) -> Option<Lane> {
        Lane::ALL.into_iter().find(|l| l.name() == name)
    }
}

/// Um bloco medido: começo e duração em segundos desde que o profiler ligou, e a
/// profundidade (um handler que dispara outro fica um nível abaixo).
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub lane: Lane,
    pub name: String,
    pub start: f64,
    pub duration: f64,
    pub depth: u32,
    /// A thread (0 = a da interface).
    pub thread: u32,
}

/// Uma amostra da pilha da thread da interface (o amostrador da plataforma): as funções, da
/// de fora (`main`) à que estava rodando.
#[derive(Clone, Debug, PartialEq)]
pub struct Stack {
    pub time: f64,
    pub frames: Vec<String>,
}

/// Guarda no máximo isto de amostras de pilha esperando a IDE buscar.
const MAX_STACKS: usize = 50_000;

/// Uma amostra da pilha (de fora para dentro).
pub fn push_stack(frames: Vec<String>) {
    if !is_on() || frames.is_empty() {
        return;
    }
    if let Ok(mut s) = STATE.lock() {
        if let Some(r) = s.as_mut() {
            if r.stacks.len() >= MAX_STACKS {
                r.stacks.remove(0);
            }
            let time = r.origin.elapsed().as_secs_f64();
            r.stacks.push(Stack { time, frames });
        }
    }
}

/// Tira as amostras de pilha desde a última vez.
pub fn take_stacks() -> Vec<Stack> {
    STATE.lock().ok().and_then(|mut s| s.as_mut().map(|r| std::mem::take(&mut r.stacks))).unwrap_or_default()
}

/// Um valor ao longo do tempo (a memória, por exemplo).
#[derive(Clone, Debug, PartialEq)]
pub struct Sample {
    pub name: String,
    pub time: f64,
    pub value: f64,
}

static ON: AtomicBool = AtomicBool::new(false);
static STATE: Mutex<Option<Recorder>> = Mutex::new(None);

/// Guarda no máximo isto de blocos esperando a IDE buscar (os mais velhos saem).
const MAX_PENDING: usize = 200_000;

struct Recorder {
    origin: Instant,
    spans: Vec<Span>,
    samples: Vec<Sample>,
    stacks: Vec<Stack>,
    dropped: u64,
}

thread_local! {
    static DEPTH: Cell<u32> = const { Cell::new(0) };
    static THREAD: Cell<u32> = const { Cell::new(u32::MAX) };
}

fn thread_index() -> u32 {
    static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
    THREAD.with(|t| {
        if t.get() == u32::MAX {
            // A primeira thread a medir com o profiler ligado costuma ser a da interface.
            let is_main = std::thread::current().name().is_none_or(|n| n == "main");
            t.set(if is_main { 0 } else { NEXT.fetch_add(1, Ordering::Relaxed) });
        }
        t.get()
    })
}

/// Liga (e zera) o profiler. No navegador, ainda não (o `Instant` do Rust não existe lá).
pub fn start() {
    if cfg!(target_arch = "wasm32") {
        return;
    }
    if let Ok(mut s) = STATE.lock() {
        *s = Some(Recorder { origin: Instant::now(), spans: Vec::new(), samples: Vec::new(), stacks: Vec::new(), dropped: 0 });
    }
    ON.store(true, Ordering::Release);
}

/// Desliga (o que não foi buscado fica para o último [`take`]).
pub fn stop() {
    ON.store(false, Ordering::Release);
}

pub fn is_on() -> bool {
    ON.load(Ordering::Relaxed)
}

/// Tira os blocos e as amostras medidos desde a última vez, e quantos blocos foram
/// descartados por falta de busca.
pub fn take() -> (Vec<Span>, Vec<Sample>, u64) {
    let Ok(mut s) = STATE.lock() else { return Default::default() };
    match s.as_mut() {
        Some(r) => (std::mem::take(&mut r.spans), std::mem::take(&mut r.samples), std::mem::take(&mut r.dropped)),
        None => Default::default(),
    }
}

/// Segundos desde que o profiler ligou.
pub fn now() -> f64 {
    STATE.lock().ok().and_then(|s| s.as_ref().map(|r| r.origin.elapsed().as_secs_f64())).unwrap_or(0.0)
}

/// Um valor no tempo (a memória do app, por exemplo).
pub fn sample(name: &str, value: f64) {
    if !is_on() {
        return;
    }
    if let Ok(mut s) = STATE.lock() {
        if let Some(r) = s.as_mut() {
            let time = r.origin.elapsed().as_secs_f64();
            r.samples.push(Sample { name: name.to_string(), time, value });
        }
    }
}

/// Um bloco já medido por fora (o backend das outras linguagens manda o começo e a duração).
pub fn record(lane: Lane, name: &str, start: f64, duration: f64) {
    if !is_on() {
        return;
    }
    push(Span { lane, name: name.to_string(), start, duration, depth: DEPTH.with(Cell::get), thread: thread_index() });
}

fn push(span: Span) {
    if let Ok(mut s) = STATE.lock() {
        if let Some(r) = s.as_mut() {
            if r.spans.len() >= MAX_PENDING {
                let extra = r.spans.len() + 1 - MAX_PENDING;
                r.spans.drain(..extra);
                r.dropped += extra as u64;
            }
            r.spans.push(span);
        }
    }
}

/// Mede até sair do escopo. Com o profiler desligado não guarda nada.
#[must_use = "o bloco termina quando a variável sai do escopo"]
pub struct Scope {
    inner: Option<(Lane, String, Instant, u32)>,
}

impl Drop for Scope {
    fn drop(&mut self) {
        let Some((lane, name, began, depth)) = self.inner.take() else { return };
        DEPTH.with(|d| d.set(depth));
        if !is_on() {
            return;
        }
        let duration = began.elapsed().as_secs_f64();
        let start = STATE.lock().ok().and_then(|s| s.as_ref().map(|r| began.saturating_duration_since(r.origin).as_secs_f64())).unwrap_or(0.0);
        push(Span { lane, name, start, duration, depth, thread: thread_index() });
    }
}

/// Começa um bloco na faixa `lane`; o nome só é montado com o profiler ligado.
pub fn scope(lane: Lane, name: impl FnOnce() -> String) -> Scope {
    if !is_on() {
        return Scope { inner: None };
    }
    let depth = DEPTH.with(|d| {
        let depth = d.get();
        d.set(depth + 1);
        depth
    });
    Scope { inner: Some((lane, name(), Instant::now(), depth)) }
}

/// Marca à mão um trecho do seu código (a faixa "Marcas"): termina quando o valor sai do
/// escopo.
pub fn profile(name: &str) -> Scope {
    scope(Lane::Mark, || name.to_string())
}

/// Os blocos e as amostras em JSON, para a IDE (`{"spans": [[faixa, nome, começo, duração,
/// profundidade, thread]...], "samples": [[nome, tempo, valor]...], "stacks": [[tempo, [funções]]...],
/// "dropped": n}`).
pub fn to_json(spans: &[Span], samples: &[Sample], stacks: &[Stack], dropped: u64) -> String {
    let mut out = String::from("{\"spans\":[");
    for (i, s) in spans.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!("[\"{}\",{},{},{},{},{}]", s.lane.name(), json_string(&s.name), s.start, s.duration, s.depth, s.thread));
    }
    out.push_str("],\"samples\":[");
    for (i, s) in samples.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!("[{},{},{}]", json_string(&s.name), s.time, s.value));
    }
    out.push_str("],\"stacks\":[");
    for (i, s) in stacks.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let frames: Vec<String> = s.frames.iter().map(|f| json_string(f)).collect();
        out.push_str(&format!("[{},[{}]]", s.time, frames.join(",")));
    }
    out.push_str(&format!("],\"dropped\":{dropped}}}"));
    out
}

fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_scopes_have_depth_and_off_records_nothing() {
        let _off = scope(Lane::Layout, || panic!("desligado não monta o nome"));
        start();
        {
            let _outer = scope(Lane::Handler, || "frm.btn_click".into());
            std::thread::sleep(std::time::Duration::from_millis(5));
            let _inner = profile("consulta");
        }
        stop();
        // Outros testes rodando ao mesmo tempo também medem: só os deste.
        let (all, _, _) = take();
        let spans: Vec<Span> = all.into_iter().filter(|s| s.name == "consulta" || s.name == "frm.btn_click").collect();
        assert_eq!(spans.len(), 2);
        // O de dentro termina primeiro.
        assert_eq!((spans[0].name.as_str(), spans[0].depth, spans[0].lane), ("consulta", 1, Lane::Mark));
        assert_eq!((spans[1].name.as_str(), spans[1].depth), ("frm.btn_click", 0));
        assert!(spans[1].duration >= 0.004 && spans[1].start <= spans[0].start);
        let stacks = [Stack { time: 0.5, frames: vec!["main".into(), "app::salvar".into()] }];
        let json = to_json(&spans, &[Sample { name: "memória".into(), time: 1.0, value: 2.0 }], &stacks, 0);
        assert!(json.starts_with("{\"spans\":[[\"mark\",\"consulta\","), "{json}");
        assert!(json.contains("[\"memória\",1,2]"), "{json}");
        assert!(json.contains("\"stacks\":[[0.5,[\"main\",\"app::salvar\"]]]"), "{json}");
    }
}
