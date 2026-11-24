//! Uma instância só do app (o `single_instance` do projeto): a segunda abertura entrega os
//! argumentos à primeira, que vem para a frente (`Ui::fire_second_instance`, o
//! `on_second_instance` do `ApplicationEvents`), e sai.
//!
//! - **Quem é a primeira:** no Windows, um mutex com nome (`Global\aylis-single-...`), por
//!   usuário (`user`: o nome leva o usuário) ou da máquina (`machine`); no Linux e no macOS, um
//!   arquivo travado (`flock`) na pasta do usuário ou em `/tmp`.
//! - **O canal dos argumentos:** só do mesmo usuário. No Windows, um pipe local
//!   (`\\.\pipe\aylis-single-...`) que recusa outra máquina e, pela segurança padrão do pipe, só
//!   aceita escrita do mesmo usuário (e do sistema e dos administradores); no Linux e no macOS,
//!   um socket na pasta do usuário (`XDG_RUNTIME_DIR`, ou `/tmp/aylis-<usuário>` com permissão
//!   só dele). Os argumentos vêm de outro processo: o app valida o que usa.
//! - **No celular e no navegador** não há (o sistema já abre uma só).
//!
//! A instância de outro usuário (`machine`) não recebe os argumentos: a segunda abertura só sai,
//! com o aviso no stderr.

use std::io::{Read, Write};
use std::sync::Mutex;

use aylis_core::SingleInstance;

/// O maior pedido aceito (os argumentos de uma abertura).
const MAX_MESSAGE: u64 = 64 * 1024;

/// Para onde vão os argumentos que chegam: o `Poster` do `Ui` (depois do `run`) ou, antes dele,
/// a fila.
static INBOX: Mutex<(Option<aylis_core::Poster>, Vec<Vec<String>>)> = Mutex::new((None, Vec::new()));

/// Na abertura, antes da janela: `true`, este processo segue (é a primeira instância, ou
/// `off`); `false`, outra instância recebeu os argumentos (ou é de outro usuário) e este deve
/// sair sem abrir nada. `app_id` separa um app do outro (o identificador do pacote).
pub fn claim_single_instance(mode: SingleInstance, app_id: &str) -> bool {
    if mode == SingleInstance::Off {
        return true;
    }
    let user = user_name();
    let app = hash(app_id.trim().to_lowercase().as_bytes());
    let channel = hash(format!("{app_id}|{user}").to_lowercase().as_bytes());
    let lock_name = match mode {
        SingleInstance::Machine => format!("aylis-single-{app:016x}"),
        _ => format!("aylis-single-{channel:016x}"),
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    if platform::lock(&lock_name, mode) {
        platform::serve(channel);
        return true;
    }
    if platform::forward(channel, &args) {
        return false;
    }
    aylis_core::stderr_line!("aylis: o app já está aberto (por outro usuário desta máquina); esta abertura sai");
    false
}

/// O `Ui` passa a receber os argumentos (o `run` chama ao começar): os que chegaram antes
/// saem agora.
pub(crate) fn attach(poster: aylis_core::Poster) {
    let Ok(mut inbox) = INBOX.lock() else { return };
    for args in std::mem::take(&mut inbox.1) {
        poster.post(move |ui| ui.fire_second_instance(args));
    }
    inbox.0 = Some(poster);
}

/// Chegaram os argumentos de outra abertura.
fn deliver(text: &str) {
    let args: Vec<String> = if text.is_empty() { Vec::new() } else { text.split('\0').map(str::to_string).collect() };
    let Ok(mut inbox) = INBOX.lock() else { return };
    match &inbox.0 {
        Some(poster) => poster.post(move |ui| ui.fire_second_instance(args)),
        None => inbox.1.push(args),
    }
}

/// O usuário deste processo (o nome do canal e da trava `user`).
fn user_name() -> String {
    std::env::var("USERNAME").or_else(|_| std::env::var("USER")).unwrap_or_default()
}

/// FNV-1a: o mesmo número nas duas aberturas (o `DefaultHasher` pode mudar entre versões).
fn hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3))
}

/// Os argumentos como vão pelo canal: separados por NUL (que um argumento não tem).
fn encode(args: &[String]) -> String {
    args.join("\0")
}

/// Lê um pedido do canal (até o limite).
fn read_message(mut stream: impl Read) -> String {
    let mut bytes = Vec::new();
    let _ = (&mut stream).take(MAX_MESSAGE).read_to_end(&mut bytes);
    String::from_utf8_lossy(&bytes).into_owned()
}

#[cfg(windows)]
mod platform {
    use windows::Win32::Foundation::{ERROR_ALREADY_EXISTS, ERROR_PIPE_CONNECTED, GetLastError, HANDLE};
    use windows::Win32::Storage::FileSystem::{FILE_FLAG_FIRST_PIPE_INSTANCE, PIPE_ACCESS_INBOUND};
    use windows::Win32::System::Pipes::{ConnectNamedPipe, CreateNamedPipeW, PIPE_READMODE_BYTE, PIPE_REJECT_REMOTE_CLIENTS, PIPE_TYPE_BYTE, PIPE_UNLIMITED_INSTANCES, PIPE_WAIT};
    use windows::Win32::System::Threading::CreateMutexW;
    use windows::core::HSTRING;

    use super::*;

    fn pipe_name(channel: u64) -> String {
        format!(r"\\.\pipe\aylis-single-{channel:016x}")
    }

    /// O mutex com nome: o primeiro a criar fica com ele até o processo acabar. No `Global\`,
    /// vale entre as sessões (o RDP e o console do mesmo usuário, ou de todos com `machine`).
    pub(super) fn lock(name: &str, _mode: SingleInstance) -> bool {
        // SAFETY: cria (ou abre) o mutex; o handle fica aberto de propósito até o fim do processo.
        unsafe {
            let created = CreateMutexW(None, false, &HSTRING::from(format!(r"Global\{name}")));
            let exists = GetLastError() == ERROR_ALREADY_EXISTS;
            match created {
                Ok(_) => !exists,
                // Sem acesso ao mutex: é de outro usuário (que o criou com a segurança dele).
                Err(_) => false,
            }
        }
    }

    fn create(name: &HSTRING, first: bool) -> Option<HANDLE> {
        let open = if first { PIPE_ACCESS_INBOUND | FILE_FLAG_FIRST_PIPE_INSTANCE } else { PIPE_ACCESS_INBOUND };
        // SAFETY: cria o pipe com um nome válido; quem recebe o handle fecha. A segurança padrão
        // do pipe dá a escrita só ao dono (este usuário), ao sistema e aos administradores.
        let handle = unsafe {
            CreateNamedPipeW(name, open, PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS, PIPE_UNLIMITED_INSTANCES, 0, MAX_MESSAGE as u32, 0, None)
        };
        (!handle.is_invalid()).then_some(handle)
    }

    /// Espera as outras aberturas numa thread (parada no `ConnectNamedPipe`). A primeira
    /// instância do pipe nasce aqui, antes de a thread começar: a segunda abertura logo depois já
    /// acha o pipe.
    pub(super) fn serve(channel: u64) {
        let name = HSTRING::from(pipe_name(channel));
        let Some(first) = create(&name, true) else { return };
        let first = first.0 as usize;
        let _ = std::thread::Builder::new().name("aylis-single-instance".into()).spawn(move || {
            let mut current = HANDLE(first as *mut _);
            loop {
                // SAFETY: o handle é um pipe criado aqui e ainda aberto.
                let connected = unsafe { ConnectNamedPipe(current, None) };
                let connected = connected.is_ok() || connected.is_err_and(|e| e.code() == ERROR_PIPE_CONNECTED.to_hresult());
                let Some(next) = create(&name, false) else { return };
                // SAFETY: o `File` fica dono do handle do pipe (e fecha ao sair do escopo).
                let file = unsafe { <std::fs::File as std::os::windows::io::FromRawHandle>::from_raw_handle(current.0) };
                current = next;
                if connected {
                    deliver(&read_message(file));
                }
            }
        });
    }

    /// Entrega os argumentos à primeira instância (que pode estar subindo: espera o pipe até 3 s).
    pub(super) fn forward(channel: u64, args: &[String]) -> bool {
        let name = pipe_name(channel);
        for _ in 0..60 {
            match std::fs::OpenOptions::new().write(true).open(&name) {
                Ok(mut pipe) => {
                    // A primeira pode vir para a frente (o Windows só deixa quem está em primeiro plano).
                    // SAFETY: sem ponteiros; só libera o primeiro plano para outro processo.
                    unsafe {
                        let _ = windows::Win32::UI::WindowsAndMessaging::AllowSetForegroundWindow(windows::Win32::UI::WindowsAndMessaging::ASFW_ANY);
                    }
                    return pipe.write_all(encode(args).as_bytes()).is_ok();
                }
                // Ocupado (231) ou ainda não criado (2): espera um pouco.
                Err(e) if matches!(e.raw_os_error(), Some(231 | 2)) => std::thread::sleep(std::time::Duration::from_millis(50)),
                // Sem acesso (5): o pipe é de outro usuário.
                Err(_) => return false,
            }
        }
        false
    }
}

#[cfg(unix)]
mod platform {
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::path::PathBuf;

    use super::*;

    /// A pasta do usuário: o `XDG_RUNTIME_DIR` (só dele) ou `/tmp/aylis-<usuário>`, criada só
    /// para ele (0700).
    fn user_dir() -> PathBuf {
        if let Some(dir) = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from).filter(|d| d.is_dir()) {
            return dir;
        }
        let dir = std::env::temp_dir().join(format!("aylis-{}", user_name()));
        let _ = std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&dir);
        let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
        dir
    }

    fn socket(channel: u64) -> PathBuf {
        user_dir().join(format!("aylis-single-{channel:016x}.sock"))
    }

    /// O arquivo travado até o fim do processo: na pasta do usuário (`user`) ou em `/tmp`
    /// (`machine`, para todos).
    pub(super) fn lock(name: &str, mode: SingleInstance) -> bool {
        let dir = if mode == SingleInstance::Machine { std::env::temp_dir() } else { user_dir() };
        let Ok(file) = std::fs::OpenOptions::new().create(true).truncate(false).write(true).open(dir.join(format!("{name}.lock"))) else {
            // Sem como travar (o arquivo é de outro usuário, sem escrita): a outra está aberta.
            return false;
        };
        if mode == SingleInstance::Machine {
            let _ = file.set_permissions(std::fs::Permissions::from_mode(0o666));
        }
        match file.try_lock() {
            Ok(()) => {
                // Fica aberto (e travado) até o processo acabar.
                std::mem::forget(file);
                true
            }
            Err(_) => false,
        }
    }

    pub(super) fn serve(channel: u64) {
        let path = socket(channel);
        // O socket de uma abertura que caiu fica no disco: a trava acima diz que não há outra.
        let _ = std::fs::remove_file(&path);
        let Ok(listener) = UnixListener::bind(&path) else { return };
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        let _ = std::thread::Builder::new().name("aylis-single-instance".into()).spawn(move || {
            for stream in listener.incoming().flatten() {
                deliver(&read_message(stream));
            }
        });
    }

    pub(super) fn forward(channel: u64, args: &[String]) -> bool {
        let path = socket(channel);
        for _ in 0..60 {
            match UnixStream::connect(&path) {
                Ok(mut stream) => return stream.write_all(encode(args).as_bytes()).is_ok(),
                Err(_) => std::thread::sleep(std::time::Duration::from_millis(50)),
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_arguments_go_through_the_channel_as_they_are() {
        let args = vec!["--abrir".to_string(), String::new(), "C:\\pasta; com ponto e vírgula\\a b.txt".to_string()];
        let text = encode(&args);
        let back: Vec<String> = text.split('\0').map(str::to_string).collect();
        assert_eq!(back, args);
        assert_eq!(hash(b"app|user"), hash(b"app|user"));
        assert_ne!(hash(b"app|user"), hash(b"app|outro"));
    }

    /// O identificador do app nos dois processos do teste abaixo (único por rodada).
    const APP_ENV: &str = "AYLIS_SINGLE_INSTANCE_TEST_APP";

    /// A segunda abertura de verdade, noutro processo (este mesmo executável de teste, rodando só
    /// o `second_instance_helper`): ela entrega os argumentos e sai; a primeira recebe pelo
    /// `Ui`, sem janela.
    #[test]
    fn the_second_instance_hands_its_arguments_to_the_first() {
        let app = format!("dev.aylis.single-test-{}", std::process::id());
        assert!(claim_single_instance(SingleInstance::User, &app), "a primeira abertura segue");
        let mut ui = aylis_core::Ui::new();
        attach(ui.poster());
        let exe = std::env::current_exe().unwrap();
        let status = std::process::Command::new(exe)
            .args(["--exact", "single_instance::tests::second_instance_helper", "--ignored", "--nocapture", "--test-threads=1"])
            .env(APP_ENV, &app)
            .status()
            .unwrap();
        assert!(status.success(), "a segunda abertura falhou: {status}");
        let started = std::time::Instant::now();
        while ui.second_instance_args().is_empty() && started.elapsed().as_secs() < 10 {
            ui.process_posted();
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        // O executável de teste recebe os argumentos do filtro: eles chegam inteiros, na ordem.
        let args = ui.second_instance_args().to_vec();
        assert!(args.iter().any(|a| a == "single_instance::tests::second_instance_helper"), "{args:?}");
        assert!(args.iter().any(|a| a == "--test-threads=1"), "{args:?}");
    }

    #[test]
    #[ignore = "a segunda abertura do teste acima, noutro processo"]
    fn second_instance_helper() {
        let Ok(app) = std::env::var(APP_ENV) else { return };
        assert!(!claim_single_instance(SingleInstance::User, &app), "com a primeira aberta, a segunda não segue");
    }
}
