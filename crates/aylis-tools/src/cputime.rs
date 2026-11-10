//! Tempo de processador de outro processo, lido no sistema (o depurador mede quanto a linha
//! gastou entre um passo e a próxima parada; o programa parado não gasta nada).
//!
//! | Sistema | Thread | Processo |
//! |---|---|---|
//! | Windows | `QueryThreadCycleTime` (ciclos, convertidos pela frequência medida) | `QueryProcessCycleTime` |
//! | Linux | `/proc/<pid>/task/<tid>/schedstat` (ns) | `/proc/<pid>/stat` (utime + stime) |
//!
//! No macOS (`thread_info`), ainda não.

use std::time::Duration;

/// O tempo de processador da thread `tid` do processo `pid` (`None`: a thread não é desse
/// processo ou o sistema não deixou ler).
pub fn thread_cpu(pid: u32, tid: u64) -> Option<Duration> {
    imp::thread_cpu(pid, tid)
}

/// O tempo de processador de todas as threads do processo.
pub fn process_cpu(pid: u32) -> Option<Duration> {
    imp::process_cpu(pid)
}

/// A memória privada do processo, em bytes (o que ele alocou): o depurador mostra a mudança
/// no passo nas linguagens sem o contador do aylis (C++, Free Pascal, C#, Java...).
pub fn process_memory(pid: u32) -> Option<u64> {
    imp::process_memory(pid)
}

#[cfg(windows)]
mod imp {
    use std::sync::OnceLock;
    use std::time::{Duration, Instant};

    unsafe extern "system" {
        fn OpenThread(access: u32, inherit: i32, tid: u32) -> isize;
        fn OpenProcess(access: u32, inherit: i32, pid: u32) -> isize;
        fn CloseHandle(handle: isize) -> i32;
        fn QueryThreadCycleTime(thread: isize, cycles: *mut u64) -> i32;
        fn QueryProcessCycleTime(process: isize, cycles: *mut u64) -> i32;
        fn GetProcessIdOfThread(thread: isize) -> u32;
        fn GetCurrentThread() -> isize;
        fn K32GetProcessMemoryInfo(process: isize, counters: *mut MemoryCounters, size: u32) -> i32;
    }
    /// O `PROCESS_MEMORY_COUNTERS_EX`.
    #[repr(C)]
    #[derive(Default)]
    struct MemoryCounters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set: usize,
        working_set: usize,
        quota_peak_paged_pool: usize,
        quota_paged_pool: usize,
        quota_peak_non_paged_pool: usize,
        quota_non_paged_pool: usize,
        pagefile_usage: usize,
        peak_pagefile_usage: usize,
        private_usage: usize,
    }
    const PROCESS_VM_READ: u32 = 0x0010;

    pub fn process_memory(pid: u32) -> Option<u64> {
        // SAFETY: o handle é fechado logo depois; a estrutura tem o tamanho que o sistema pede.
        unsafe {
            let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ, 0, pid);
            if process == 0 {
                return None;
            }
            let mut c = MemoryCounters { cb: std::mem::size_of::<MemoryCounters>() as u32, ..MemoryCounters::default() };
            let ok = K32GetProcessMemoryInfo(process, &mut c, c.cb) != 0;
            CloseHandle(process);
            ok.then_some(c.private_usage as u64)
        }
    }
    const THREAD_QUERY_LIMITED_INFORMATION: u32 = 0x0800;
    const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;

    /// Ciclos por segundo: medidos uma vez, girando 20 ms nesta thread.
    fn hz() -> f64 {
        static HZ: OnceLock<f64> = OnceLock::new();
        *HZ.get_or_init(|| {
            let cycles = || {
                let mut c = 0u64;
                // SAFETY: a pseudo-handle da própria thread, sem fechar.
                unsafe { QueryThreadCycleTime(GetCurrentThread(), &mut c) };
                c
            };
            let (start, c0) = (Instant::now(), cycles());
            let mut x = 0u64;
            while start.elapsed() < Duration::from_millis(20) {
                x = std::hint::black_box(x.wrapping_add(1));
            }
            let elapsed = start.elapsed().as_secs_f64();
            ((cycles() - c0) as f64 / elapsed).max(1.0e8)
        })
    }

    fn duration(cycles: u64) -> Duration {
        Duration::from_secs_f64(cycles as f64 / hz())
    }

    pub fn thread_cpu(pid: u32, tid: u64) -> Option<Duration> {
        let tid = u32::try_from(tid).ok()?;
        // SAFETY: chamadas simples do Windows; o handle é fechado logo em seguida.
        unsafe {
            let h = OpenThread(THREAD_QUERY_LIMITED_INFORMATION, 0, tid);
            if h == 0 {
                return None;
            }
            let mut cycles = 0u64;
            let ok = GetProcessIdOfThread(h) == pid && QueryThreadCycleTime(h, &mut cycles) != 0;
            CloseHandle(h);
            ok.then(|| duration(cycles))
        }
    }

    pub fn process_cpu(pid: u32) -> Option<Duration> {
        // SAFETY: como acima.
        unsafe {
            let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if h == 0 {
                return None;
            }
            let mut cycles = 0u64;
            let ok = QueryProcessCycleTime(h, &mut cycles) != 0;
            CloseHandle(h);
            ok.then(|| duration(cycles))
        }
    }
}

#[cfg(target_os = "linux")]
mod imp {
    use std::time::Duration;

    pub fn thread_cpu(pid: u32, tid: u64) -> Option<Duration> {
        // `/proc/<n>/task/<tid>` também abre com `n` sendo outra thread do processo: a
        // thread tem de ser deste processo (o `Tgid`).
        let status = std::fs::read_to_string(format!("/proc/{pid}/task/{tid}/status")).ok()?;
        let tgid = status.lines().find_map(|l| l.strip_prefix("Tgid:")).and_then(|v| v.trim().parse::<u32>().ok())?;
        if tgid != pid {
            return None;
        }
        let text = std::fs::read_to_string(format!("/proc/{pid}/task/{tid}/schedstat")).ok()?;
        let ns = text.split_whitespace().next()?.parse::<u64>().ok()?;
        Some(Duration::from_nanos(ns))
    }

    pub fn process_memory(pid: u32) -> Option<u64> {
        // `statm`: o segundo campo é o residente; o sexto, os dados (o que o processo alocou), em páginas.
        let text = std::fs::read_to_string(format!("/proc/{pid}/statm")).ok()?;
        let pages = text.split_whitespace().nth(5)?.parse::<u64>().ok()?;
        Some(pages * 4096)
    }

    pub fn process_cpu(pid: u32) -> Option<Duration> {
        let text = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
        // Depois do nome (entre parênteses, pode ter espaço): utime e stime são o 12º e o 13º.
        let rest = &text[text.rfind(')')? + 1..];
        let fields: Vec<&str> = rest.split_whitespace().collect();
        let ticks = fields.get(11)?.parse::<u64>().ok()? + fields.get(12)?.parse::<u64>().ok()?;
        // O relógio dos ticks é de 100 por segundo no Linux de hoje.
        Some(Duration::from_millis(ticks * 10))
    }
}

#[cfg(not(any(windows, target_os = "linux")))]
mod imp {
    use std::time::Duration;

    pub fn thread_cpu(_pid: u32, _tid: u64) -> Option<Duration> {
        None
    }

    pub fn process_cpu(_pid: u32) -> Option<Duration> {
        None
    }

    pub fn process_memory(_pid: u32) -> Option<u64> {
        None
    }
}

/// O texto do fim da linha: o tempo de processador, a espera (relógio − processador) só
/// quando passa de 500 ms, e a memória (a mudança no que está alocado).
pub fn line_note(cpu: Option<Duration>, wall: Duration, memory: Option<i64>) -> String {
    let mut parts = Vec::new();
    match cpu {
        Some(cpu) => {
            parts.push(format!("⏱ {}", short_time(cpu)));
            let waiting = wall.saturating_sub(cpu);
            if waiting > Duration::from_millis(500) {
                parts.push(format!("espera {}", short_time(waiting)));
            }
        }
        None => parts.push(format!("⏱ {} (relógio)", short_time(wall))),
    }
    if let Some(bytes) = memory {
        parts.push(format!("{}{}", if bytes >= 0 { "+" } else { "−" }, short_bytes(bytes.unsigned_abs())));
    }
    parts.join("  ·  ")
}

/// Um tempo curto: `340 µs`, `12,0 ms`, `1,60 s`.
pub fn short_time(d: Duration) -> String {
    let us = d.as_secs_f64() * 1e6;
    if us < 1000.0 {
        format!("{us:.0} µs")
    } else if us < 1e6 {
        format!("{:.1} ms", us / 1000.0).replace('.', ",")
    } else {
        format!("{:.2} s", us / 1e6).replace('.', ",")
    }
}

fn short_bytes(b: u64) -> String {
    match b {
        0..1024 => format!("{b} B"),
        1024..1_048_576 => format!("{:.1} KB", b as f64 / 1024.0).replace('.', ","),
        _ => format!("{:.1} MB", b as f64 / 1_048_576.0).replace('.', ","),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_this_process_and_a_thread_of_it() {
        let pid = std::process::id();
        let before = process_cpu(pid);
        if cfg!(any(windows, target_os = "linux")) {
            let before = before.expect("processo");
            // Gasta processador numa thread e lê a dela pelo id do sistema.
            let (tx, rx) = std::sync::mpsc::channel();
            let (go, wait) = std::sync::mpsc::channel::<()>();
            let worker = std::thread::spawn(move || {
                tx.send(os_thread_id()).unwrap();
                wait.recv().unwrap();
                let start = std::time::Instant::now();
                let mut x = 0u64;
                while start.elapsed() < Duration::from_millis(60) {
                    x = std::hint::black_box(x.wrapping_add(1));
                }
                tx.send(0).unwrap();
                // Espera a leitura antes de sair (o id some com a thread).
                wait.recv().unwrap();
            });
            let tid = rx.recv().unwrap();
            let t0 = thread_cpu(pid, tid).expect("thread");
            go.send(()).unwrap();
            rx.recv().unwrap();
            let t1 = thread_cpu(pid, tid).expect("thread");
            // Uma thread de outro processo não é aceita (no Linux o caminho nem existe).
            assert_eq!(thread_cpu(pid.wrapping_add(1), tid), None);
            go.send(()).unwrap();
            worker.join().ok();
            let spent = t1 - t0;
            assert!((Duration::from_millis(30)..Duration::from_millis(200)).contains(&spent), "a thread gastou {spent:?}");
            let after = process_cpu(pid).unwrap();
            assert!(after > before, "{before:?} → {after:?}");
        }
    }

    #[cfg(windows)]
    fn os_thread_id() -> u64 {
        unsafe extern "system" {
            fn GetCurrentThreadId() -> u32;
        }
        // SAFETY: sem argumentos.
        unsafe { GetCurrentThreadId() as u64 }
    }

    #[cfg(target_os = "linux")]
    fn os_thread_id() -> u64 {
        unsafe extern "C" {
            fn gettid() -> i32;
        }
        // SAFETY: sem argumentos.
        unsafe { gettid() as u64 }
    }

    #[cfg(not(any(windows, target_os = "linux")))]
    fn os_thread_id() -> u64 {
        0
    }

    #[test]
    fn the_note_shows_wait_only_past_half_a_second() {
        assert_eq!(line_note(Some(Duration::from_micros(340)), Duration::from_millis(2), None), "⏱ 340 µs");
        assert_eq!(line_note(Some(Duration::from_millis(12)), Duration::from_millis(1612), Some(2048)), "⏱ 12,0 ms  ·  espera 1,60 s  ·  +2,0 KB");
        assert_eq!(line_note(None, Duration::from_millis(3), Some(-10)), "⏱ 3,0 ms (relógio)  ·  −10 B");
    }
}

#[cfg(all(test, any(windows, target_os = "linux")))]
mod memory_tests {
    /// A memória privada deste processo cresce quando ele aloca (e é tocada).
    #[test]
    fn process_memory_sees_an_allocation() {
        let pid = std::process::id();
        let before = super::process_memory(pid).unwrap();
        let block = vec![7u8; 64 * 1024 * 1024];
        let after = super::process_memory(pid).unwrap();
        std::hint::black_box(&block);
        assert!(after >= before + 32 * 1024 * 1024, "{before} → {after}");
    }
}
