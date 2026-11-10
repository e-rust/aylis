//! Trabalho dividido entre os núcleos: filtrar e ordenar milhões de registros sem deixar
//! a interface parada segundos num núcleo só.

use std::cmp::Ordering;

/// Abaixo disto por thread, dividir custa mais do que ganha.
const MIN_PER_THREAD: usize = 50_000;

fn threads(len: usize) -> usize {
    let cores = std::thread::available_parallelism().map_or(1, |n| n.get());
    cores.min(len / MIN_PER_THREAD).max(1)
}

fn join<T>(handle: std::thread::ScopedJoinHandle<'_, T>) -> T {
    handle.join().unwrap_or_else(|e| std::panic::resume_unwind(e))
}

/// `f` em cada item, com o resultado na mesma ordem.
pub(crate) fn map<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let t = threads(items.len());
    if t == 1 {
        return items.iter().map(f).collect();
    }
    let size = items.len().div_ceil(t);
    let f = &f;
    std::thread::scope(|s| {
        let handles: Vec<_> = items.chunks(size).map(|chunk| s.spawn(move || chunk.iter().map(f).collect::<Vec<R>>())).collect();
        let mut out = Vec::with_capacity(items.len());
        for h in handles {
            out.extend(join(h));
        }
        out
    })
}

/// Ordena com `cmp`, que precisa desempatar tudo (a ordem final não depende
/// da divisão entre as threads).
pub(crate) fn sort<T: Copy + Send + Sync>(order: &mut Vec<T>, cmp: impl Fn(&T, &T) -> Ordering + Sync) {
    let t = threads(order.len());
    if t == 1 {
        order.sort_unstable_by(cmp);
        return;
    }
    let size = order.len().div_ceil(t);
    let cmp = &cmp;
    std::thread::scope(|s| {
        for chunk in order.chunks_mut(size) {
            s.spawn(move || chunk.sort_unstable_by(cmp));
        }
    });
    // Junta os pedaços ordenados dois a dois, cada par numa thread.
    let mut runs: Vec<Vec<T>> = order.chunks(size).map(<[T]>::to_vec).collect();
    while runs.len() > 1 {
        let mut pairs = Vec::new();
        let mut it = runs.into_iter();
        while let Some(a) = it.next() {
            pairs.push((a, it.next()));
        }
        runs = std::thread::scope(|s| {
            let handles: Vec<_> = pairs
                .into_iter()
                .map(|(a, b)| {
                    s.spawn(move || match b {
                        Some(b) => merge(a, b, cmp),
                        None => a,
                    })
                })
                .collect();
            handles.into_iter().map(join).collect()
        });
    }
    *order = runs.pop().unwrap_or_default();
}

fn merge<T: Copy>(a: Vec<T>, b: Vec<T>, cmp: &impl Fn(&T, &T) -> Ordering) -> Vec<T> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if cmp(&b[j], &a[i]) == Ordering::Less {
            out.push(b[j]);
            j += 1;
        } else {
            out.push(a[i]);
            i += 1;
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort_matches_the_single_thread_order() {
        let mut order: Vec<u32> = (0..400_000u32).map(|i| i.wrapping_mul(2654435761) % 1_000_003).collect();
        let mut expected = order.clone();
        expected.sort_unstable();
        super::sort(&mut order, |a, b| a.cmp(b));
        assert_eq!(order, expected);
        let doubled = super::map(&order, |x| x * 2);
        assert_eq!(doubled[1234], order[1234] * 2);
    }
}
