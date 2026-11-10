//! Ler e gravar os arquivos que os diálogos devolvem, em qualquer plataforma.
//!
//! No desktop o caminho é um arquivo comum. No navegador não há disco: o arquivo escolhido
//! fica na memória (`memory:/1/foto.png`) e salvar baixa o arquivo (`download:/relatorio.pdf`).
//! No Android, salvar grava no documento que o usuário escolheu (`content://...`). Use
//! [`read`] e [`write`] no lugar de `std::fs` com os caminhos dos diálogos (`OpenDialog`,
//! `SaveDialog`, `FileInput`, `FileDropZone`) e o código vale para todas.
//!
//! ```
//! let caminho = aylis_core::files::put_memory("nota.txt", b"oi".to_vec());
//! assert_eq!(aylis_core::files::read(&caminho).unwrap(), b"oi");
//! assert_eq!(aylis_core::files::file_name(&caminho), "nota.txt");
//! ```

use std::collections::HashMap;
use std::io;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

/// Quem lê e grava os caminhos de um esquema (`content://`, `download:/`). A plataforma
/// registra os dela com [`set_scheme`].
pub trait FileScheme: Send + Sync {
    fn read(&self, path: &str) -> io::Result<Vec<u8>>;
    fn write(&self, path: &str, data: &[u8]) -> io::Result<()>;
}

const MEMORY: &str = "memory:/";

static SCHEMES: RwLock<Vec<(&'static str, Arc<dyn FileScheme>)>> = RwLock::new(Vec::new());
static STORE: Mutex<Option<HashMap<String, Arc<Vec<u8>>>>> = Mutex::new(None);
static NEXT: AtomicU64 = AtomicU64::new(1);

/// Liga `handler` aos caminhos que começam com `prefix` (troca o anterior, se houver).
pub fn set_scheme(prefix: &'static str, handler: impl FileScheme + 'static) {
    if let Ok(mut s) = SCHEMES.write() {
        s.retain(|(p, _)| *p != prefix);
        s.push((prefix, Arc::new(handler)));
    }
}

fn scheme(path: &str) -> Option<Arc<dyn FileScheme>> {
    SCHEMES.read().ok()?.iter().find(|(p, _)| path.starts_with(p)).map(|(_, h)| h.clone())
}

/// Guarda `data` na memória e devolve o caminho (`memory:/N/nome`): o arquivo escolhido no
/// navegador, ou um arquivo de teste.
pub fn put_memory(name: &str, data: Vec<u8>) -> String {
    let path = format!("{MEMORY}{}/{}", NEXT.fetch_add(1, Ordering::Relaxed), file_name(name));
    if let Ok(mut store) = STORE.lock() {
        store.get_or_insert_with(HashMap::new).insert(path.clone(), Arc::new(data));
    }
    path
}

/// Solta da memória um arquivo de [`put_memory`] (os outros caminhos: nada).
pub fn forget(path: &str) {
    if let Ok(mut store) = STORE.lock() {
        if let Some(s) = store.as_mut() {
            s.remove(path);
        }
    }
}

/// O conteúdo do arquivo `path`.
pub fn read(path: &str) -> io::Result<Vec<u8>> {
    if path.starts_with(MEMORY) {
        let store = STORE.lock().map_err(|_| io::Error::other("arquivos na memória"))?;
        return store.as_ref().and_then(|s| s.get(path)).map(|d| d.as_ref().clone()).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, path.to_string()));
    }
    match scheme(path) {
        Some(h) => h.read(path),
        None => std::fs::read(path),
    }
}

/// Grava `data` em `path` (no navegador, um caminho `download:/` baixa o arquivo).
pub fn write(path: &str, data: &[u8]) -> io::Result<()> {
    if path.starts_with(MEMORY) {
        let mut store = STORE.lock().map_err(|_| io::Error::other("arquivos na memória"))?;
        store.get_or_insert_with(HashMap::new).insert(path.to_string(), Arc::new(data.to_vec()));
        return Ok(());
    }
    match scheme(path) {
        Some(h) => h.write(path, data),
        None => std::fs::write(path, data),
    }
}

/// O tamanho em bytes (`None` se não der para saber sem ler).
pub fn size(path: &str) -> Option<u64> {
    if path.starts_with(MEMORY) {
        return STORE.lock().ok()?.as_ref()?.get(path).map(|d| d.len() as u64);
    }
    if scheme(path).is_some() {
        return None;
    }
    std::fs::metadata(path).ok().map(|m| m.len())
}

/// O nome do arquivo, sem a pasta (vale para todos os esquemas).
pub fn file_name(path: &str) -> &str {
    let path = path.trim_end_matches(['/', '\\']);
    // content://.../document/primary%3ADownload%2Ffoto.png: o nome vem depois do último %2F.
    let last = path.rsplit(['/', '\\']).next().unwrap_or(path);
    last.rsplit("%2F").next().unwrap_or(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Upper;

    impl FileScheme for Upper {
        fn read(&self, path: &str) -> io::Result<Vec<u8>> {
            Ok(path.to_uppercase().into_bytes())
        }
        fn write(&self, _: &str, data: &[u8]) -> io::Result<()> {
            if data.is_empty() { Err(io::Error::other("vazio")) } else { Ok(()) }
        }
    }

    #[test]
    fn memory_files_and_schemes() {
        let p = put_memory("C:\\fotos\\a.png", vec![1, 2, 3]);
        assert!(p.starts_with("memory:/") && p.ends_with("/a.png"));
        assert_eq!(read(&p).unwrap(), [1, 2, 3]);
        assert_eq!(size(&p), Some(3));
        write(&p, &[9]).unwrap();
        assert_eq!(read(&p).unwrap(), [9]);
        forget(&p);
        assert!(read(&p).is_err());

        set_scheme("teste://", Upper);
        assert_eq!(read("teste://x").unwrap(), b"TESTE://X");
        assert!(write("teste://x", b"").is_err());
        assert_eq!(file_name("content://com.android/document/primary%3ADownload%2Ffoto.png"), "foto.png");
    }
}
