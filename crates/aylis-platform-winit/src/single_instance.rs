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
//!
//! O projeto pode dar o nome exato da trava (`single_instance_name`: o mesmo de um programa
//! anterior, para os dois não rodarem juntos), fazer a segunda abertura sair calada
//! (`single_instance_second = quiet`: sem entregar nada e sem trazer a primeira) e soltar a trava
//! antes de relançar a si mesmo ([`release_single_instance`]).

use std::io::{Read, Write};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use aylis_core::{SecondInstance, SingleInstance};

/// O maior pedido aceito (os argumentos de uma abertura).
const MAX_MESSAGE: u64 = 64 * 1024;

/// Para onde vão os argumentos que chegam: o `Poster` do `Ui` (depois do `run`) ou, antes dele,
/// a fila.
static INBOX: Mutex<(Option<aylis_core::Poster>, Vec<Vec<String>>)> = Mutex::new((None, Vec::new()));

/// O canal servido por este processo (para a soltura acordar quem espera nele).
static SERVED: Mutex<Option<u64>> = Mutex::new(None);

/// A trava foi solta ([`release_single_instance`]): o canal para de receber.
static RELEASED: AtomicBool = AtomicBool::new(false);

/// A thread do canal já fechou depois da soltura.
static STOPPED: AtomicBool = AtomicBool::new(false);

/// O maior nome de trava aceito (o do mutex do Windows vai até 260).
const MAX_NAME: usize = 200;

/// Na abertura, antes da janela: `true`, este processo segue (é a primeira instância, ou
/// `off`); `false`, outra instância recebeu os argumentos (ou é de outro usuário) e este deve
/// sair sem abrir nada. `app_id` separa um app do outro (o identificador do pacote).
pub fn claim_single_instance(mode: SingleInstance, app_id: &str) -> bool {
    claim_single_instance_with(mode, app_id, "", SecondInstance::Forward)
}

/// Como [`claim_single_instance`], com o nome da trava e o que a segunda abertura faz.
///
/// - `name`: o nome exato da trava (o `single_instance_name` do projeto); vazio, o gerado do
///   `app_id`. No Windows é o nome do mutex, como está: com `Global\` vale entre as sessões
///   (o de um programa anterior que use o mesmo nome impede os dois juntos), sem prefixo vale só
///   na sessão. No Linux e no macOS, o nome do arquivo travado (`/` e `\` viram `-`).
/// - `second`: `Forward` entrega os argumentos à primeira (que vem para a frente); `Quiet` sai
///   calada, sem entregar nada (e a primeira nem abre o canal).
pub fn claim_single_instance_with(mode: SingleInstance, app_id: &str, name: &str, second: SecondInstance) -> bool {
    if mode == SingleInstance::Off {
        return true;
    }
    let user = user_name();
    let name = name.trim();
    let (lock_name, channel) = if name.is_empty() {
        let app = hash(app_id.trim().to_lowercase().as_bytes());
        let channel = hash(format!("{app_id}|{user}").to_lowercase().as_bytes());
        let lock = match mode {
            SingleInstance::Machine => format!("aylis-single-{app:016x}"),
            _ => format!("aylis-single-{channel:016x}"),
        };
        (lock, channel)
    } else {
        // O nome do projeto vale como está (só sem o que não cabe num nome de trava).
        let clean: String = name.chars().filter(|c| !c.is_control()).take(MAX_NAME).collect();
        (clean, hash(format!("{name}|{user}").to_lowercase().as_bytes()))
    };
    RELEASED.store(false, Ordering::SeqCst);
    if platform::lock(&lock_name, mode, !name.is_empty()) {
        if second == SecondInstance::Forward {
            platform::serve(channel);
            if let Ok(mut served) = SERVED.lock() {
                *served = Some(channel);
            }
        }
        return true;
    }
    if second == SecondInstance::Quiet {
        return false;
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    if platform::forward(channel, &args) {
        return false;
    }
    aylis_core::stderr_line!("aylis: o app já está aberto (por outro usuário desta máquina, ou por outro programa com a mesma trava); esta abertura sai");
    false
}

/// Solta a trava da instância única antes de o app relançar a si mesmo (a atualização que
/// troca o exe e abre o novo): a trava e o canal dos argumentos saem, e a próxima abertura
/// passa a ser a primeira. Esta instância continua rodando, sem receber as outras aberturas.
/// Sem a trava, não faz nada.
pub fn release_single_instance() {
    if RELEASED.swap(true, Ordering::SeqCst) {
        return;
    }
    platform::unlock();
    let served = SERVED.lock().ok().and_then(|mut s| s.take());
    if let Some(channel) = served {
        platform::stop_serving(channel);
    }
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

    /// O handle do mutex deste processo (aberto até o fim, ou até a soltura).
    static MUTEX: std::sync::Mutex<Option<usize>> = std::sync::Mutex::new(None);

    /// O mutex com nome: o primeiro a criar fica com ele até o processo acabar. No `Global\`,
    /// vale entre as sessões (o RDP e o console do mesmo usuário, ou de todos com `machine`).
    /// `exact`: o nome do projeto, como está (com ou sem o `Global\`).
    pub(super) fn lock(name: &str, _mode: SingleInstance, exact: bool) -> bool {
        let full = if exact { name.to_string() } else { format!(r"Global\{name}") };
        // SAFETY: cria (ou abre) o mutex; o handle fica aberto de propósito até o fim do processo
        // (ou até o `unlock`), e o da segunda abertura também, que logo sai.
        unsafe {
            let created = CreateMutexW(None, false, &HSTRING::from(full));
            let exists = GetLastError() == ERROR_ALREADY_EXISTS;
            match created {
                Ok(handle) if !exists => {
                    if let Ok(mut m) = MUTEX.lock() {
                        *m = Some(handle.0 as usize);
                    }
                    true
                }
                Ok(_) => false,
                // Sem acesso ao mutex: é de outro usuário (que o criou com a segurança dele).
                Err(_) => false,
            }
        }
    }

    /// Fecha o mutex: com o último handle fechado, o nome fica livre para a próxima abertura.
    pub(super) fn unlock() {
        let handle = MUTEX.lock().ok().and_then(|mut m| m.take());
        if let Some(handle) = handle {
            // SAFETY: o handle é o do mutex criado no `lock`, fechado uma vez só (saiu do `MUTEX`).
            let _ = unsafe { windows::Win32::Foundation::CloseHandle(HANDLE(handle as *mut _)) };
        }
    }

    /// Acorda a thread do canal (parada no `ConnectNamedPipe`) para ela fechar o pipe e sair: a
    /// próxima abertura cria o pipe dela como o primeiro.
    pub(super) fn stop_serving(channel: u64) {
        let _ = std::fs::OpenOptions::new().write(true).open(pipe_name(channel));
        // Espera a thread fechar o pipe (até 1 s): quem relança logo depois já acha o nome livre.
        for _ in 0..50 {
            if super::STOPPED.load(Ordering::SeqCst) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
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
                // Solta (`release_single_instance`): fecha o pipe sem abrir o próximo.
                if super::RELEASED.load(Ordering::SeqCst) {
                    // SAFETY: o handle é o pipe criado aqui, fechado uma vez só.
                    let _ = unsafe { windows::Win32::Foundation::CloseHandle(current) };
                    super::STOPPED.store(true, Ordering::SeqCst);
                    return;
                }
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

    /// O arquivo travado deste processo (até o fim, ou até a soltura).
    static LOCKED: std::sync::Mutex<Option<std::fs::File>> = std::sync::Mutex::new(None);

    /// O arquivo travado até o fim do processo: na pasta do usuário (`user`) ou em `/tmp`
    /// (`machine`, para todos). O nome do projeto (`exact`) sem as barras.
    pub(super) fn lock(name: &str, mode: SingleInstance, exact: bool) -> bool {
        let dir = if mode == SingleInstance::Machine { std::env::temp_dir() } else { user_dir() };
        let name = if exact { name.replace(['/', '\\'], "-") } else { name.to_string() };
        let Ok(file) = std::fs::OpenOptions::new().create(true).truncate(false).write(true).open(dir.join(format!("{name}.lock"))) else {
            // Sem como travar (o arquivo é de outro usuário, sem escrita): a outra está aberta.
            return false;
        };
        if mode == SingleInstance::Machine {
            let _ = file.set_permissions(std::fs::Permissions::from_mode(0o666));
        }
        match file.try_lock() {
            Ok(()) => {
                // Fica aberto (e travado) até o processo acabar, ou até o `unlock`.
                if let Ok(mut l) = LOCKED.lock() {
                    *l = Some(file);
                }
                true
            }
            Err(_) => false,
        }
    }

    /// Fecha o arquivo travado: a trava sai com ele.
    pub(super) fn unlock() {
        let file = LOCKED.lock().ok().and_then(|mut l| l.take());
        drop(file);
    }

    /// Acorda a thread do canal (parada no `accept`) para ela sair, e tira o socket do disco: a
    /// próxima abertura cria o dela.
    pub(super) fn stop_serving(channel: u64) {
        let path = socket(channel);
        let _ = UnixStream::connect(&path);
        let _ = std::fs::remove_file(&path);
    }

    pub(super) fn serve(channel: u64) {
        let path = socket(channel);
        // O socket de uma abertura que caiu fica no disco: a trava acima diz que não há outra.
        let _ = std::fs::remove_file(&path);
        let Ok(listener) = UnixListener::bind(&path) else { return };
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        let _ = std::thread::Builder::new().name("aylis-single-instance".into()).spawn(move || {
            for stream in listener.incoming().flatten() {
                // Solta (`release_single_instance`): o socket fecha com a thread.
                if super::RELEASED.load(Ordering::SeqCst) {
                    super::STOPPED.store(true, Ordering::SeqCst);
                    return;
                }
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

    /// A trava é uma por processo (a soltura para o canal de quem travou): os testes que travam
    /// rodam um por vez.
    static SERIAL: Mutex<()> = Mutex::new(());

    /// A segunda abertura de verdade, noutro processo (este mesmo executável de teste, rodando só
    /// o `second_instance_helper`): ela entrega os argumentos e sai; a primeira recebe pelo
    /// `Ui`, sem janela.
    #[test]
    fn the_second_instance_hands_its_arguments_to_the_first() {
        let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
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

    /// A trava com o nome do projeto, o que a outra abertura espera dela.
    const NAME_ENV: &str = "AYLIS_SINGLE_INSTANCE_TEST_NAME";
    const EXPECT_ENV: &str = "AYLIS_SINGLE_INSTANCE_TEST_EXPECT";

    /// Roda o `named_instance_helper` noutro processo: ele tenta a trava `name` calado e confere
    /// se seguiu como `expect` diz.
    fn other_opening(name: &str, expect: bool) -> std::time::Duration {
        let started = std::time::Instant::now();
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "single_instance::tests::named_instance_helper", "--ignored", "--nocapture", "--test-threads=1"])
            .env(NAME_ENV, name)
            .env(EXPECT_ENV, if expect { "1" } else { "0" })
            .status()
            .unwrap();
        assert!(status.success(), "a outra abertura não fez o esperado ({expect}): {status}");
        started.elapsed()
    }

    /// O nome da trava escolhido pelo projeto, a segunda abertura calada e a soltura: com a
    /// trava, a outra abertura sai (sem esperar o canal, que a calada nem abre); solta, a outra
    /// passa a ser a primeira. Um teste só, porque a trava é do processo.
    #[test]
    fn the_named_lock_keeps_the_quiet_second_out_until_it_is_released() {
        let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let name = format!("dev.aylis.single-named-{}", std::process::id());
        assert!(claim_single_instance_with(SingleInstance::User, "outro.app", &name, SecondInstance::Quiet), "a primeira segue");
        let took = other_opening(&name, false);
        // A calada não espera o canal (a que entrega espera até 3 s).
        assert!(took < std::time::Duration::from_secs(20), "{took:?}");
        release_single_instance();
        other_opening(&name, true);
    }

    #[test]
    #[ignore = "a outra abertura do teste acima, noutro processo"]
    fn named_instance_helper() {
        let (Ok(name), Ok(expect)) = (std::env::var(NAME_ENV), std::env::var(EXPECT_ENV)) else { return };
        let claimed = claim_single_instance_with(SingleInstance::User, "outro.app", &name, SecondInstance::Quiet);
        assert_eq!(claimed, expect == "1");
    }
}
