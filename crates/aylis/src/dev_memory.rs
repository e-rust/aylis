//! Só com `dev` (o app que a IDE compila para rodar e depurar; a publicação nunca leva):
//! conta os bytes alocados, para o depurador mostrar no fim da linha quanto o passo
//! alocou (`+12,0 KB`). A IDE lê `AYLIS_DEV_LIVE_BYTES` pelo depurador ao continuar e ao
//! parar. Custa duas operações atômicas por alocação.
//!
//! Um app com o próprio `#[global_allocator]` não compila com `dev`: tire o dele em
//! `cfg(not(feature = "dev"))`.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicI64, Ordering};

/// Os bytes alocados agora (o que foi pedido menos o que foi devolvido).
#[unsafe(no_mangle)]
pub static AYLIS_DEV_LIVE_BYTES: AtomicI64 = AtomicI64::new(0);

struct Counting;

// SAFETY: repassa tudo ao alocador do sistema; só conta.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: o mesmo contrato do chamador.
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            AYLIS_DEV_LIVE_BYTES.fetch_add(layout.size() as i64, Ordering::Relaxed);
        }
        p
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: o mesmo contrato do chamador.
        let p = unsafe { System.alloc_zeroed(layout) };
        if !p.is_null() {
            AYLIS_DEV_LIVE_BYTES.fetch_add(layout.size() as i64, Ordering::Relaxed);
        }
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: o mesmo contrato do chamador.
        unsafe { System.dealloc(ptr, layout) };
        AYLIS_DEV_LIVE_BYTES.fetch_sub(layout.size() as i64, Ordering::Relaxed);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: o mesmo contrato do chamador.
        let p = unsafe { System.realloc(ptr, layout, new_size) };
        if !p.is_null() {
            AYLIS_DEV_LIVE_BYTES.fetch_add(new_size as i64 - layout.size() as i64, Ordering::Relaxed);
        }
        p
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Os bytes alocados agora (para testes e diagnósticos).
pub fn live_bytes() -> i64 {
    AYLIS_DEV_LIVE_BYTES.load(Ordering::Relaxed)
}
