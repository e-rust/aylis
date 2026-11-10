//! NFC no desktop: um leitor USB pelo PC/SC (ACR122U e parecidos), com as tags do tipo 2
//! (NTAG, Mifare Ultralight), que são as comuns em adesivos e cartões de NFC. A leitura roda
//! numa thread: espera a tag encostar, lê o id e a memória (os comandos "pseudo-APDU" do
//! PC/SC: `FF CA` e `FF B0`) e grava com `FF D6`, página por página.

use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use aylis_core::ndef;
use aylis_core::platform::{NfcEvent, NfcService, NfcTag};
use pcsc::{Card, Context, Protocols, ReaderState, Scope, ShareMode, State};

#[derive(Default)]
struct Shared {
    events: Vec<NfcEvent>,
    pending_write: Option<Vec<u8>>,
}

pub(crate) struct DesktopNfc {
    shared: Arc<Mutex<Shared>>,
    reading: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
    /// Há leitor? Consultado de tempos em tempos (o `available` é chamado a cada timer).
    readers: Mutex<(Instant, bool)>,
}

impl DesktopNfc {
    pub(crate) fn new() -> Self {
        Self { shared: Arc::default(), reading: Arc::default(), thread: None, readers: Mutex::new((Instant::now() - Duration::from_secs(60), false)) }
    }
}

fn reader_names(ctx: &Context) -> Vec<CString> {
    let mut buffer = vec![0u8; ctx.list_readers_len().unwrap_or(0).max(1)];
    ctx.list_readers(&mut buffer).map(|names| names.map(CString::from).collect()).unwrap_or_default()
}

impl NfcService for DesktopNfc {
    fn available(&self) -> bool {
        let Ok(mut r) = self.readers.lock() else { return false };
        if r.0.elapsed() > Duration::from_secs(3) {
            *r = (Instant::now(), Context::establish(Scope::User).is_ok_and(|ctx| !reader_names(&ctx).is_empty()));
        }
        r.1
    }

    fn set_reading(&mut self, on: bool) {
        self.reading.store(on, Ordering::SeqCst);
        if !on {
            if let Some(t) = self.thread.take() {
                let _ = t.join();
            }
            return;
        }
        if self.thread.as_ref().is_some_and(|t| !t.is_finished()) {
            return;
        }
        let (shared, reading) = (self.shared.clone(), self.reading.clone());
        self.thread = std::thread::Builder::new().name("aylis-nfc".into()).spawn(move || read_loop(&shared, &reading)).ok();
    }

    fn write(&mut self, message: Vec<u8>) {
        if let Ok(mut s) = self.shared.lock() {
            s.pending_write = Some(message);
        }
    }

    fn cancel_write(&mut self) {
        if let Ok(mut s) = self.shared.lock() {
            s.pending_write = None;
        }
    }

    fn poll(&mut self) -> Vec<NfcEvent> {
        self.shared.lock().map(|mut s| std::mem::take(&mut s.events)).unwrap_or_default()
    }
}

impl Drop for DesktopNfc {
    fn drop(&mut self) {
        self.set_reading(false);
    }
}

fn push(shared: &Mutex<Shared>, event: NfcEvent) {
    if let Ok(mut s) = shared.lock() {
        s.events.push(event);
    }
}

/// Espera as tags enquanto `reading`; cada tag é lida (e gravada, se pedido) uma vez até sair.
fn read_loop(shared: &Mutex<Shared>, reading: &AtomicBool) {
    let ctx = match Context::establish(Scope::User) {
        Ok(c) => c,
        Err(e) => return push(shared, NfcEvent::Error(format!("PC/SC: {e}"))),
    };
    let mut states: Vec<ReaderState> = Vec::new();
    while reading.load(Ordering::SeqCst) {
        // Leitores ligados ou tirados no meio: a lista é refeita.
        let names = reader_names(&ctx);
        if names.is_empty() {
            std::thread::sleep(Duration::from_millis(500));
            continue;
        }
        if states.len() != names.len() || states.iter().zip(&names).any(|(s, n)| s.name() != n.as_c_str()) {
            states = names.into_iter().map(|n| ReaderState::new(n, State::UNAWARE)).collect();
        }
        match ctx.get_status_change(Duration::from_millis(300), &mut states) {
            Ok(()) => {}
            Err(pcsc::Error::Timeout) => continue,
            Err(_) => {
                std::thread::sleep(Duration::from_millis(300));
                continue;
            }
        }
        for state in &mut states {
            let changed = state.event_state().contains(State::CHANGED);
            let present = state.event_state().contains(State::PRESENT) && !state.current_state().contains(State::PRESENT);
            if changed && present {
                match ctx.connect(state.name(), ShareMode::Shared, Protocols::ANY) {
                    Ok(card) => handle_card(shared, &card),
                    Err(e) => push(shared, NfcEvent::Error(format!("PC/SC: {e}"))),
                }
            }
            state.sync_current_state();
        }
    }
}

/// Manda um comando e devolve a resposta sem o `90 00` (ou o erro com o código).
fn transmit(card: &Card, apdu: &[u8]) -> Result<Vec<u8>, String> {
    let mut buffer = [0u8; 300];
    let answer = card.transmit(apdu, &mut buffer).map_err(|e| format!("PC/SC: {e}"))?;
    match answer {
        [data @ .., 0x90, 0x00] => Ok(data.to_vec()),
        [.., a, b] => Err(format!("a tag recusou o comando ({a:02X} {b:02X})")),
        _ => Err("a tag não respondeu".into()),
    }
}

/// Lê 4 páginas (16 bytes) a partir de `page`.
fn read_pages(card: &Card, page: u8) -> Result<Vec<u8>, String> {
    transmit(card, &[0xFF, 0xB0, 0x00, page, 0x10])
}

fn handle_card(shared: &Mutex<Shared>, card: &Card) {
    let id = match transmit(card, &[0xFF, 0xCA, 0x00, 0x00, 0x00]) {
        Ok(id) => id,
        Err(e) => return push(shared, NfcEvent::Error(e)),
    };
    // O "capability container" (página 3): o tamanho da área de dados e se aceita gravar.
    let cc = read_pages(card, 3).ok().filter(|p| p.first() == Some(&0xE1));
    let size = cc.as_ref().map_or(0, |c| c[2] as usize * 8);
    let writable = cc.as_ref().is_some_and(|c| c[3] & 0xF0 == 0);
    if let Some(message) = shared.lock().ok().and_then(|mut s| s.pending_write.take()) {
        let memory = ndef::to_type2_memory(&message);
        let result = if cc.is_none() {
            Err("a tag não é do tipo 2 (NTAG, Ultralight) ou não tem NDEF".to_string())
        } else if !writable {
            Err("a tag é só de leitura".to_string())
        } else if memory.len() > size {
            Err(format!("a mensagem não cabe na tag ({} de {size} bytes)", memory.len()))
        } else {
            memory.chunks(4).enumerate().try_for_each(|(i, chunk)| {
                let mut apdu = vec![0xFF, 0xD6, 0x00, 4 + i as u8, 0x04];
                apdu.extend_from_slice(chunk);
                transmit(card, &apdu).map(|_| ())
            })
        };
        push(shared, if let Err(e) = result { NfcEvent::Error(e) } else { NfcEvent::Written });
    }
    // A memória a partir da página 4, até achar o fim da mensagem (o TLV `FE`).
    let mut memory = Vec::new();
    let limit = if size > 0 { size } else { 1024 };
    let mut records = Vec::new();
    while memory.len() < limit {
        match read_pages(card, 4 + (memory.len() / 4) as u8) {
            Ok(chunk) if !chunk.is_empty() => memory.extend_from_slice(&chunk),
            _ => break,
        }
        if let Some(message) = ndef::from_type2_memory(&memory) {
            records = ndef::decode(&message);
            break;
        }
        // Começa com o terminador: a tag está vazia.
        if memory.iter().find(|b| **b != 0) == Some(&0xFE) {
            break;
        }
    }
    push(shared, NfcEvent::Tag(NfcTag { id: ndef::id_text(&id), tech: "NfcA".into(), records, writable }));
}
