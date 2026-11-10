//! A amostragem da pilha da thread da interface, para o profiler da IDE (a árvore de
//! chamadas e o flame graph), no Windows x64: uma thread pausa a da interface de 2 em 2 ms,
//! lê os registradores e desenrola a pilha pelas tabelas de unwind do executável
//! (`RtlVirtualUnwind`), sem alocar nada enquanto ela está pausada (o alocador pode estar
//! preso nela). Os nomes das funções vêm do PDB pelo `dbghelp`, depois, fora da pausa.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{
    CONTEXT, CONTEXT_CONTROL_AMD64, CONTEXT_INTEGER_AMD64, GetThreadContext, RtlLookupFunctionEntry, RtlVirtualUnwind, SYMBOL_INFOW, SYMOPT_DEFERRED_LOADS,
    SYMOPT_UNDNAME, SymFromAddrW, SymInitializeW, SymSetOptions, UNW_FLAG_NHANDLER,
};
use windows::Win32::System::Threading::{
    GetCurrentProcess, GetCurrentThreadId, OpenThread, ResumeThread, SuspendThread, THREAD_GET_CONTEXT, THREAD_QUERY_INFORMATION, THREAD_SUSPEND_RESUME,
};

static RUNNING: AtomicBool = AtomicBool::new(false);

/// Quantas funções por amostra, no máximo.
const DEPTH: usize = 96;

/// O `CONTEXT` precisa de alinhamento de 16 bytes.
#[repr(C, align(16))]
struct Aligned(CONTEXT);

/// Começa a amostrar a thread que chamou (a da interface), enquanto o profiler estiver ligado.
pub fn start() {
    if RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }
    let id = unsafe { GetCurrentThreadId() };
    let Ok(thread) = (unsafe { OpenThread(THREAD_SUSPEND_RESUME | THREAD_GET_CONTEXT | THREAD_QUERY_INFORMATION, false, id) }) else {
        RUNNING.store(false, Ordering::SeqCst);
        return;
    };
    // O HANDLE não é Send: passa como número.
    let raw = thread.0 as isize;
    let spawned = std::thread::Builder::new().name("aylis-sampler".into()).spawn(move || {
        let thread = HANDLE(raw as *mut _);
        let mut symbols = Symbols::new();
        let mut frames = [0u64; DEPTH];
        while RUNNING.load(Ordering::Relaxed) && aylis_core::profile::is_on() {
            let n = unsafe { capture(thread, &mut frames) };
            if n > 0 {
                // De fora para dentro, sem as funções do sistema antes do `main`.
                let names: Vec<String> = frames[..n].iter().rev().map(|&a| symbols.name(a)).collect();
                aylis_core::profile::push_stack(names);
            }
            std::thread::sleep(Duration::from_millis(2));
        }
        unsafe {
            let _ = CloseHandle(thread);
        }
        RUNNING.store(false, Ordering::SeqCst);
    });
    if spawned.is_err() {
        RUNNING.store(false, Ordering::SeqCst);
    }
}

/// Para de amostrar.
pub fn stop() {
    RUNNING.store(false, Ordering::SeqCst);
}

/// Pausa a thread, guarda os endereços de retorno da pilha dela e solta. Sem alocar.
///
/// # Safety
/// `thread` é um handle aberto com suspender, ler o contexto e consultar.
unsafe fn capture(thread: HANDLE, out: &mut [u64; DEPTH]) -> usize {
    if unsafe { SuspendThread(thread) } == u32::MAX {
        return 0;
    }
    let mut context = Aligned(unsafe { std::mem::zeroed() });
    context.0.ContextFlags = CONTEXT_CONTROL_AMD64 | CONTEXT_INTEGER_AMD64;
    let mut n = 0;
    if unsafe { GetThreadContext(thread, &mut context.0) }.is_ok() {
        let c = &mut context.0;
        while n < out.len() && c.Rip != 0 && c.Rsp != 0 {
            out[n] = c.Rip;
            n += 1;
            let mut image_base = 0u64;
            let entry = unsafe { RtlLookupFunctionEntry(c.Rip, &mut image_base, None) };
            if entry.is_null() {
                // Uma função folha (sem tabela): o retorno está no topo da pilha.
                c.Rip = unsafe { *(c.Rsp as *const u64) };
                c.Rsp += 8;
            } else {
                let mut handler_data = std::ptr::null_mut();
                let mut establisher = 0u64;
                unsafe { RtlVirtualUnwind(UNW_FLAG_NHANDLER, image_base, c.Rip, entry, c, &mut handler_data, &mut establisher, None) };
            }
        }
    }
    unsafe { ResumeThread(thread) };
    n
}

/// Os nomes das funções pelo endereço (o PDB do executável, pelo `dbghelp`), com cache.
struct Symbols {
    ready: bool,
    cache: HashMap<u64, String>,
}

impl Symbols {
    fn new() -> Self {
        // O PDB fica ao lado do executável (o do cargo também, em target/.../deps).
        let dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_string_lossy().to_string())).unwrap_or_default();
        let path = windows::core::HSTRING::from(dir);
        let ready = unsafe {
            SymSetOptions(SYMOPT_UNDNAME | SYMOPT_DEFERRED_LOADS);
            SymInitializeW(GetCurrentProcess(), &path, true).is_ok()
        };
        Self { ready, cache: HashMap::new() }
    }

    fn name(&mut self, address: u64) -> String {
        if let Some(name) = self.cache.get(&address) {
            return name.clone();
        }
        let name = self.lookup(address).unwrap_or_else(|| format!("0x{address:x}"));
        self.cache.insert(address, name.clone());
        name
    }

    fn lookup(&self, address: u64) -> Option<String> {
        if !self.ready {
            return None;
        }
        // O SYMBOL_INFOW com espaço para o nome logo depois dele.
        const MAX: usize = 512;
        #[repr(C)]
        struct Buffer {
            info: SYMBOL_INFOW,
            rest: [u16; MAX],
        }
        let mut buffer: Buffer = unsafe { std::mem::zeroed() };
        buffer.info.SizeOfStruct = std::mem::size_of::<SYMBOL_INFOW>() as u32;
        buffer.info.MaxNameLen = MAX as u32;
        let mut displacement = 0u64;
        unsafe { SymFromAddrW(GetCurrentProcess(), address, Some(&mut displacement), &mut buffer.info) }.ok()?;
        let len = (buffer.info.NameLen as usize).min(MAX);
        let name = unsafe { std::slice::from_raw_parts(buffer.info.Name.as_ptr(), len) };
        Some(String::from_utf16_lossy(name))
    }
}
