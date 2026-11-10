//! A amostragem da pilha (o profiler da IDE): com a thread ocupada numa função conhecida, as
//! amostras trazem o nome dela, abaixo de quem a chamou.

#![cfg(all(windows, target_arch = "x86_64"))]

use std::time::{Duration, Instant};

#[inline(never)]
fn aylis_busy_marker(until: Instant) -> u64 {
    let mut x = 0u64;
    while Instant::now() < until {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        std::hint::black_box(x);
    }
    x
}

#[inline(never)]
fn aylis_caller_marker() -> u64 {
    // Algo depois da chamada: otimizada, ela viraria um salto e a função sumiria da pilha.
    let x = aylis_busy_marker(Instant::now() + Duration::from_millis(400));
    std::hint::black_box(x).wrapping_add(1)
}

#[test]
fn the_sampler_sees_the_busy_function_and_its_caller() {
    aylis_core::profile::start();
    aylis_platform_winit::sampler::start();
    std::hint::black_box(aylis_caller_marker());
    aylis_platform_winit::sampler::stop();
    std::thread::sleep(Duration::from_millis(50));
    let stacks = aylis_core::profile::take_stacks();
    aylis_core::profile::stop();
    assert!(stacks.len() >= 20, "poucas amostras: {}", stacks.len());
    let hits = stacks
        .iter()
        .filter(|s| {
            let busy = s.frames.iter().position(|f| f.contains("aylis_busy_marker"));
            let caller = s.frames.iter().position(|f| f.contains("aylis_caller_marker"));
            matches!((caller, busy), (Some(c), Some(b)) if c < b)
        })
        .count();
    assert!(hits * 2 >= stacks.len(), "{hits} de {} amostras na função; uma: {:?}", stacks.len(), stacks.first().map(|s| &s.frames));
}
