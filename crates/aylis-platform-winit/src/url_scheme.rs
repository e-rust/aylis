//! O prefixo de URL próprio no Windows (o componente `UrlScheme`).
//!
//! - **Registro:** `HKCU\Software\Classes\<prefixo>` com `URL Protocol` e o
//!   `shell\open\command` apontando para este executável com `"%1"`: só do usuário, sem
//!   pedir administrador. Quem chama `meuapp://...` (o navegador, o Executar, outro app) abre
//!   o app com o endereço na linha de comando.
//! - **O app já aberto:** a cópia nova, aberta pelo link, repassa o endereço à que já está
//!   rodando por um pipe local (`\\.\pipe\aylis-links-<este executável e usuário>`, que
//!   recusa conexão de outra máquina) e fecha; a aberta entrega no `on_link` e vem para a
//!   frente. O pipe só existe com um `UrlScheme` ativo.

use std::io::{Read, Write};
use std::sync::{Arc, Mutex, OnceLock};

use windows::Win32::Foundation::{ERROR_PIPE_CONNECTED, HANDLE};
use windows::Win32::Storage::FileSystem::{FILE_FLAG_FIRST_PIPE_INSTANCE, PIPE_ACCESS_INBOUND};
use windows::Win32::System::Pipes::{ConnectNamedPipe, CreateNamedPipeW, PIPE_READMODE_BYTE, PIPE_REJECT_REMOTE_CLIENTS, PIPE_TYPE_BYTE, PIPE_UNLIMITED_INSTANCES, PIPE_WAIT};
use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ, RRF_RT_REG_SZ, RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegGetValueW, RegSetValueExW,
};
use windows::core::{HSTRING, PCWSTR};

/// O maior pedido que o pipe aceita (vários links de uma vez).
const MAX_MESSAGE: u64 = 64 * 1024;

fn exe() -> Result<String, String> {
    std::env::current_exe().map(|p| p.to_string_lossy().into_owned()).map_err(|e| e.to_string())
}

fn key(scheme: &str) -> String {
    format!("Software\\Classes\\{scheme}")
}

/// Grava o texto `value` em `HKCU\<path>` (`name` vazio: o valor padrão da chave).
fn set(path: &str, name: &str, value: &str) -> Result<(), String> {
    let mut key = HKEY::default();
    let wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
    // SAFETY: chamadas do registro com ponteiros válidos durante a chamada; a chave é fechada
    // no fim. Os bytes são os do `wide`, que vive até o fim do bloco.
    unsafe {
        RegCreateKeyExW(HKEY_CURRENT_USER, &HSTRING::from(path), None, PCWSTR::null(), REG_OPTION_NON_VOLATILE, KEY_WRITE, None, &mut key, None)
            .ok()
            .map_err(|e| e.message())?;
        let bytes = std::slice::from_raw_parts(wide.as_ptr().cast::<u8>(), wide.len() * 2);
        let result = RegSetValueExW(key, &HSTRING::from(name), None, REG_SZ, Some(bytes)).ok().map_err(|e| e.message());
        let _ = RegCloseKey(key);
        result
    }
}

/// O texto padrão de `HKCU\<path>`.
fn get(path: &str) -> Option<String> {
    let path = HSTRING::from(path);
    let mut size = 0u32;
    // SAFETY: a primeira chamada só mede; a segunda escreve em `buffer`, do tamanho medido.
    unsafe {
        RegGetValueW(HKEY_CURRENT_USER, &path, PCWSTR::null(), RRF_RT_REG_SZ, None, None, Some(&mut size)).ok().ok()?;
        let mut buffer = vec![0u16; (size as usize).div_ceil(2)];
        RegGetValueW(HKEY_CURRENT_USER, &path, PCWSTR::null(), RRF_RT_REG_SZ, None, Some(buffer.as_mut_ptr().cast()), Some(&mut size)).ok().ok()?;
        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        Some(String::from_utf16_lossy(&buffer[..len]))
    }
}

pub(crate) fn register(scheme: &str, description: &str) -> Result<(), String> {
    let exe = exe()?;
    let base = key(scheme);
    let description = if description.trim().is_empty() { scheme } else { description.trim() };
    set(&base, "", &format!("URL:{description}"))?;
    set(&base, "URL Protocol", "")?;
    set(&format!("{base}\\DefaultIcon"), "", &format!("\"{exe}\",0"))?;
    set(&format!("{base}\\shell\\open\\command"), "", &format!("\"{exe}\" \"%1\""))
}

/// O prefixo abre este executável?
pub(crate) fn registered(scheme: &str) -> bool {
    let (Some(command), Ok(exe)) = (get(&format!("{}\\shell\\open\\command", key(scheme))), exe()) else { return false };
    command.to_lowercase().contains(&exe.to_lowercase())
}

pub(crate) fn unregister(scheme: &str) -> Result<(), String> {
    if get(&format!("{}\\shell\\open\\command", key(scheme))).is_none() {
        return Ok(());
    }
    if !registered(scheme) {
        return Err(aylis_core::messages::tf("o prefixo {0} está registrado por outro programa", &[&scheme]));
    }
    // SAFETY: apaga a árvore da chave do usuário; o caminho é uma string válida.
    unsafe { RegDeleteTreeW(HKEY_CURRENT_USER, &HSTRING::from(key(scheme))).ok().map_err(|e| e.message()) }
}

/// O nome do pipe: um por executável e usuário (outra cópia do mesmo app, do mesmo usuário).
fn pipe_name() -> String {
    let who = format!("{}|{}", exe().unwrap_or_default().to_lowercase(), std::env::var("USERNAME").unwrap_or_default().to_lowercase());
    // FNV-1a: o mesmo número nas duas cópias (o `DefaultHasher` pode mudar entre versões).
    let hash = who.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3));
    format!(r"\\.\pipe\aylis-links-{hash:016x}")
}

type Receivers = Arc<Mutex<Vec<Box<dyn Fn(String) + Send>>>>;

/// Passa a receber os links das outras cópias (o servidor do pipe sobe na primeira vez).
pub(crate) fn receive(on_link: Box<dyn Fn(String) + Send>) {
    static RECEIVERS: OnceLock<Receivers> = OnceLock::new();
    let mut started = false;
    let receivers = RECEIVERS.get_or_init(|| {
        started = true;
        Arc::new(Mutex::new(Vec::new()))
    });
    if let Ok(mut list) = receivers.lock() {
        list.push(on_link);
    }
    if started {
        let receivers = receivers.clone();
        let _ = std::thread::Builder::new().name("aylis-links".into()).spawn(move || serve(&receivers));
    }
}

/// Uma instância do pipe (`first`: a primeira, que falha se outra cópia já tem o nome).
fn create(name: &HSTRING, first: bool) -> Option<HANDLE> {
    let open = if first { PIPE_ACCESS_INBOUND | FILE_FLAG_FIRST_PIPE_INSTANCE } else { PIPE_ACCESS_INBOUND };
    // SAFETY: cria o pipe com um nome válido; quem recebe o handle fecha.
    let handle = unsafe {
        CreateNamedPipeW(name, open, PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS, PIPE_UNLIMITED_INSTANCES, 0, MAX_MESSAGE as u32, 0, None)
    };
    (!handle.is_invalid()).then_some(handle)
}

/// Espera as cópias novas. A próxima instância do pipe nasce antes de a atual ser lida, para
/// não haver um instante sem pipe (a cópia nova acharia que não há outra aberta). Parado, a
/// thread dorme no `ConnectNamedPipe`.
fn serve(receivers: &Receivers) {
    let name = HSTRING::from(pipe_name());
    // Já existe (outra cópia aberta primeiro ficou com ele): esta não recebe.
    let Some(mut current) = create(&name, true) else { return };
    loop {
        // SAFETY: o handle é um pipe criado aqui e ainda aberto.
        let connected = unsafe { ConnectNamedPipe(current, None) };
        let connected = connected.is_ok() || connected.is_err_and(|e| e.code() == ERROR_PIPE_CONNECTED.to_hresult());
        let Some(next) = create(&name, false) else { return };
        // SAFETY: o `File` fica dono do handle do pipe (e fecha ao sair do escopo).
        let mut file = unsafe { <std::fs::File as std::os::windows::io::FromRawHandle>::from_raw_handle(current.0) };
        current = next;
        if !connected {
            continue;
        }
        let mut text = String::new();
        let _ = (&mut file).take(MAX_MESSAGE).read_to_string(&mut text);
        drop(file);
        let Ok(list) = receivers.lock() else { return };
        for url in text.lines().map(str::trim).filter(|l| l.contains(':')) {
            for on_link in list.iter() {
                on_link(url.to_string());
            }
        }
    }
}

/// A cópia aberta pelo link: entrega os `links` à cópia que já está aberta. `true`: entregue
/// (esta pode fechar); `false`: não há outra aberta (esta segue e recebe ela mesma).
pub(crate) fn forward(links: &[String]) -> bool {
    let name = pipe_name();
    for _ in 0..20 {
        match std::fs::OpenOptions::new().write(true).open(&name) {
            Ok(mut pipe) => {
                // A aberta pode vir para a frente (o Windows só deixa quem está em primeiro plano).
                // SAFETY: sem ponteiros; só libera o primeiro plano para outro processo.
                unsafe {
                    let _ = windows::Win32::UI::WindowsAndMessaging::AllowSetForegroundWindow(windows::Win32::UI::WindowsAndMessaging::ASFW_ANY);
                }
                return pipe.write_all(links.join("\n").as_bytes()).is_ok();
            }
            // Ocupado com outra cópia: espera um pouco.
            Err(e) if e.raw_os_error() == Some(231) => std::thread::sleep(std::time::Duration::from_millis(50)),
            Err(_) => return false,
        }
    }
    false
}
