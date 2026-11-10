//! Fontes de arquivo (TTF, OTF, TTC) usadas pelo app além das do sistema: as do projeto
//! (`assets/fonts/`) ou registradas por código. O registro é do processo inteiro: os
//! renderizadores (Skia, Direct2D, navegador) conferem a [`generation`] e carregam as novas.
//!
//! ```ignore
//! let family = aylis_core::fonts::register(std::fs::read("Inter.ttf")?);   // Some("Inter")
//! // depois: FontSpec::new("Inter", 14.0)
//! ```

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Uma fonte registrada.
#[derive(Clone, Debug)]
pub struct FontFile {
    /// Família (lida do próprio arquivo).
    pub family: String,
    pub data: Arc<Vec<u8>>,
}

static FONTS: Mutex<Vec<FontFile>> = Mutex::new(Vec::new());
static GENERATION: AtomicU64 = AtomicU64::new(0);

/// Registra uma fonte (o arquivo inteiro). Devolve a família, ou `None` se não for uma
/// fonte. Registrar o mesmo arquivo de novo não duplica.
pub fn register(data: Vec<u8>) -> Option<String> {
    let family = family_name(&data)?;
    let mut fonts = FONTS.lock().ok()?;
    if fonts.iter().any(|f| f.family == family && f.data.len() == data.len() && *f.data == data) {
        return Some(family);
    }
    fonts.push(FontFile { family: family.clone(), data: Arc::new(data) });
    GENERATION.fetch_add(1, Ordering::SeqCst);
    Some(family)
}

/// As fontes registradas.
pub fn registered() -> Vec<FontFile> {
    FONTS.lock().map(|f| f.clone()).unwrap_or_default()
}

/// As famílias registradas, sem repetir.
pub fn families() -> Vec<String> {
    let mut list: Vec<String> = registered().into_iter().map(|f| f.family).collect();
    list.sort();
    list.dedup();
    list
}

/// Muda a cada fonte nova: os renderizadores recarregam quando ela muda.
pub fn generation() -> u64 {
    GENERATION.load(Ordering::SeqCst)
}

fn u16_at(d: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*d.get(i)?, *d.get(i + 1)?]))
}

fn u32_at(d: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_be_bytes([*d.get(i)?, *d.get(i + 1)?, *d.get(i + 2)?, *d.get(i + 3)?]))
}

/// A família de um arquivo de fonte (tabela `name`: a família "tipográfica" se houver,
/// senão a comum). Num TTC, a da primeira fonte.
pub fn family_name(data: &[u8]) -> Option<String> {
    let mut offset = 0usize;
    if data.get(0..4) == Some(b"ttcf") {
        offset = u32_at(data, 12)? as usize;
    }
    let tag = u32_at(data, offset)?;
    // TrueType (0x00010000, "true") ou OpenType com CFF ("OTTO").
    if tag != 0x0001_0000 && tag != u32::from_be_bytes(*b"OTTO") && tag != u32::from_be_bytes(*b"true") {
        return None;
    }
    let tables = u16_at(data, offset + 4)? as usize;
    let name_table = (0..tables).find_map(|i| {
        let rec = offset + 12 + i * 16;
        (data.get(rec..rec + 4)? == b"name").then(|| u32_at(data, rec + 8)).flatten()
    })? as usize;
    let count = u16_at(data, name_table + 2)? as usize;
    let strings = name_table + u16_at(data, name_table + 4)? as usize;
    let mut best: Option<(u8, String)> = None;
    for i in 0..count {
        let rec = name_table + 6 + i * 12;
        let (platform, encoding, language, name_id) = (u16_at(data, rec)?, u16_at(data, rec + 2)?, u16_at(data, rec + 4)?, u16_at(data, rec + 6)?);
        if name_id != 1 && name_id != 16 {
            continue;
        }
        let (len, off) = (u16_at(data, rec + 8)? as usize, u16_at(data, rec + 10)? as usize);
        let raw = data.get(strings + off..strings + off + len)?;
        let text = match (platform, encoding) {
            (3, 1) | (3, 10) | (0, _) => {
                let units: Vec<u16> = raw.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
                String::from_utf16_lossy(&units)
            }
            (1, 0) => raw.iter().map(|&b| b as char).collect(),
            _ => continue,
        };
        // Prioridade: típográfica (16) em inglês do Windows, depois a comum (1).
        let english = language == 0x0409 || platform != 3;
        let score = (if name_id == 16 { 2 } else { 0 }) + u8::from(english) + u8::from(platform == 3);
        if best.as_ref().is_none_or(|(s, _)| score > *s) && !text.trim().is_empty() {
            best = Some((score, text.trim().to_string()));
        }
    }
    best.map(|(_, t)| t)
}
