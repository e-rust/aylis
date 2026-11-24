//! Iniciar com o sistema ([`aylis_core::Ui::set_start_with_login`]): o app abre sozinho quando o
//! usuário entra. O mecanismo é do framework e a decisão é do app (uma opção nas preferências
//! dele); só o usuário atual, sem administrador.
//!
//! - **Windows:** o valor do app na chave `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`,
//!   com o caminho deste exe entre aspas.
//! - **Linux:** o `<app>.desktop` em `~/.config/autostart` (ou `$XDG_CONFIG_HOME/autostart`), o
//!   autostart do freedesktop que o GNOME, o KDE, o Cinnamon e o Xfce leem.
//! - **macOS:** o agente de login `~/Library/LaunchAgents/<app>.plist` (`RunAtLoad`), que o
//!   `launchd` abre no próximo login (conferir num Mac: `docs/testes-em-dispositivos.md`).
//! - **Celular e navegador:** não existe (o erro diz).
//!
//! O nome do app vem do projeto ([`set_app_identity`]); sem projeto, o nome do exe. Ligado quer
//! dizer que o registro aponta para este exe: o de uma cópia em outra pasta conta como
//! desligado, e ligar de novo passa a apontar para esta.

use std::sync::Mutex;

/// O identificador do app (o nome do pacote) e o nome para as pessoas.
static IDENTITY: Mutex<Option<(String, String)>> = Mutex::new(None);

/// O app diz quem é (o `Application` e o `aylis-host` com o projeto, antes do `run`): o nome do
/// valor no registro, do `.desktop` e do agente de login. `id` só com letras, números, `.`,
/// `-` e `_` (o resto vira `-`).
pub fn set_app_identity(id: &str, name: &str) {
    let id = clean_id(id);
    if let Ok(mut identity) = IDENTITY.lock() {
        *identity = (!id.is_empty()).then(|| (id, name.trim().to_string()));
    }
}

/// O identificador só com letras, números, `.`, `-` e `_` (vira nome de arquivo e de valor).
fn clean_id(id: &str) -> String {
    let id: String = id.trim().chars().map(|c| if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') { c } else { '-' }).collect();
    id.trim_matches(|c| c == '-' || c == '.').to_string()
}

/// O identificador e o nome do app: os do projeto, ou o nome do exe.
#[cfg_attr(any(web, mobile), allow(dead_code))]
fn identity() -> (String, String) {
    if let Some(identity) = IDENTITY.lock().ok().and_then(|i| i.clone()) {
        return identity;
    }
    let stem = std::env::current_exe().ok().and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned())).unwrap_or_else(|| "app".into());
    (clean_id(&stem), stem)
}

/// O app abre sozinho quando o usuário entra no sistema (o registro aponta para este exe).
pub fn start_with_login() -> bool {
    #[cfg(not(any(web, mobile)))]
    {
        let Ok(exe) = std::env::current_exe() else { return false };
        return platform::registered(&identity().0, &exe.to_string_lossy());
    }
    #[allow(unreachable_code)]
    false
}

/// Liga (`true`) ou desliga a abertura do app com o login. Desligar o que não estava ligado
/// não é erro.
pub fn set_start_with_login(on: bool) -> Result<(), String> {
    #[cfg(not(any(web, mobile)))]
    {
        let exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let (id, name) = identity();
        let result = if on { platform::register(&id, &name, &exe.to_string_lossy()) } else { platform::unregister(&id) };
        return result.map_err(|e| aylis_core::messages::tf("Não foi possível gravar o início com o sistema: {0}", &[&e]));
    }
    #[allow(unreachable_code)]
    {
        let _ = on;
        Err(aylis_core::messages::t("Iniciar com o sistema não existe nesta plataforma."))
    }
}

#[cfg(windows)]
mod platform {
    use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
    use windows::Win32::System::Registry::{
        HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_OPTION_NON_VOLATILE, REG_SZ, RRF_RT_REG_SZ, RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegGetValueW, RegSetValueExW,
    };
    use windows::core::{HSTRING, PCWSTR};

    const RUN: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";

    /// O comando gravado: o caminho do exe entre aspas (com espaço no caminho, sem aspas o
    /// Windows tentaria abrir só o começo).
    fn command(exe: &str) -> String {
        format!("\"{exe}\"")
    }

    pub(super) fn register(id: &str, _name: &str, exe: &str) -> Result<(), String> {
        let mut key = HKEY::default();
        let wide: Vec<u16> = command(exe).encode_utf16().chain(std::iter::once(0)).collect();
        // SAFETY: chamadas do registro com ponteiros válidos durante a chamada; a chave é fechada
        // no fim. Os bytes são os do `wide`, que vive até o fim do bloco.
        unsafe {
            RegCreateKeyExW(HKEY_CURRENT_USER, &HSTRING::from(RUN), None, PCWSTR::null(), REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, None, &mut key, None)
                .ok()
                .map_err(|e| e.message())?;
            let bytes = std::slice::from_raw_parts(wide.as_ptr().cast::<u8>(), wide.len() * 2);
            let result = RegSetValueExW(key, &HSTRING::from(id), None, REG_SZ, Some(bytes)).ok().map_err(|e| e.message());
            let _ = RegCloseKey(key);
            result
        }
    }

    pub(super) fn unregister(id: &str) -> Result<(), String> {
        let mut key = HKEY::default();
        // SAFETY: abre a chave `Run` do usuário para apagar o valor do app e a fecha no fim.
        unsafe {
            RegCreateKeyExW(HKEY_CURRENT_USER, &HSTRING::from(RUN), None, PCWSTR::null(), REG_OPTION_NON_VOLATILE, KEY_SET_VALUE, None, &mut key, None)
                .ok()
                .map_err(|e| e.message())?;
            let deleted = RegDeleteValueW(key, &HSTRING::from(id));
            let _ = RegCloseKey(key);
            if deleted == ERROR_FILE_NOT_FOUND { Ok(()) } else { deleted.ok().map_err(|e| e.message()) }
        }
    }

    /// O valor do app na chave `Run` (`None`: não há).
    fn value(id: &str) -> Option<String> {
        let (path, name) = (HSTRING::from(RUN), HSTRING::from(id));
        let mut size = 0u32;
        // SAFETY: a primeira chamada só mede; a segunda escreve em `buffer`, do tamanho medido.
        unsafe {
            RegGetValueW(HKEY_CURRENT_USER, &path, &name, RRF_RT_REG_SZ, None, None, Some(&mut size)).ok().ok()?;
            let mut buffer = vec![0u16; (size as usize).div_ceil(2)];
            RegGetValueW(HKEY_CURRENT_USER, &path, &name, RRF_RT_REG_SZ, None, Some(buffer.as_mut_ptr().cast()), Some(&mut size)).ok().ok()?;
            let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
            Some(String::from_utf16_lossy(&buffer[..len]))
        }
    }

    pub(super) fn registered(id: &str, exe: &str) -> bool {
        value(id).is_some_and(|v| v.trim().trim_matches('"').eq_ignore_ascii_case(exe))
    }
}

#[cfg(all(unix, not(any(web, mobile))))]
mod platform {
    use std::path::PathBuf;

    /// Onde fica o arquivo do app: o `.desktop` do autostart (Linux) ou o agente de login
    /// (macOS).
    fn file(id: &str) -> Option<PathBuf> {
        let home = std::env::var_os("HOME").map(PathBuf::from)?;
        #[cfg(target_os = "macos")]
        return Some(home.join("Library/LaunchAgents").join(format!("{id}.plist")));
        #[cfg(not(target_os = "macos"))]
        {
            let config = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from).filter(|p| p.is_absolute()).unwrap_or_else(|| home.join(".config"));
            Some(config.join("autostart").join(format!("{id}.desktop")))
        }
    }

    /// O texto do arquivo, apontando para `exe`.
    pub(super) fn contents(id: &str, name: &str, exe: &str) -> String {
        #[cfg(target_os = "macos")]
        {
            let _ = name;
            let xml = |s: &str| s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;");
            format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n\t<key>Label</key>\n\t<string>{}</string>\n\t<key>ProgramArguments</key>\n\t<array>\n\t\t<string>{}</string>\n\t</array>\n\t<key>RunAtLoad</key>\n\t<true/>\n</dict>\n</plist>\n",
                xml(id),
                xml(exe)
            )
        }
        #[cfg(not(target_os = "macos"))]
        {
            // O `Exec` entre aspas, com `"`, `` ` ``, `$` e `\` escapados (a especificação do
            // `.desktop`); o nome numa linha só.
            let quoted: String = exe.chars().flat_map(|c| if matches!(c, '"' | '`' | '$' | '\\') { vec!['\\', c] } else { vec![c] }).collect();
            let name = if name.is_empty() { id } else { name };
            let name: String = name.chars().filter(|c| !c.is_control()).collect();
            format!("[Desktop Entry]\nType=Application\nName={name}\nExec=\"{quoted}\"\nX-GNOME-Autostart-enabled=true\n")
        }
    }

    pub(super) fn register(id: &str, name: &str, exe: &str) -> Result<(), String> {
        let path = file(id).ok_or("sem HOME")?;
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        std::fs::write(&path, contents(id, name, exe)).map_err(|e| e.to_string())
    }

    pub(super) fn unregister(id: &str) -> Result<(), String> {
        let Some(path) = file(id) else { return Ok(()) };
        match std::fs::remove_file(&path) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.to_string()),
            _ => Ok(()),
        }
    }

    /// A linha que diz o exe no arquivo: o `Exec=` (Linux) ou o `<string>` do programa
    /// (macOS). O nome do app pode ter mudado desde a gravação; o exe é o que conta.
    fn exe_line(id: &str, exe: &str) -> String {
        let text = contents(id, "", exe);
        let wanted = |l: &&str| if cfg!(target_os = "macos") { l.trim().starts_with("<string>") && !l.contains(id) } else { l.starts_with("Exec=") };
        text.lines().find(wanted).unwrap_or_default().trim().to_string()
    }

    pub(super) fn registered(id: &str, exe: &str) -> bool {
        let Some(text) = file(id).and_then(|p| std::fs::read_to_string(p).ok()) else { return false };
        let line = exe_line(id, exe);
        !line.is_empty() && text.lines().any(|t| t.trim() == line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Liga, confere, desliga: o registro do usuário (ou o arquivo) com um nome único, apagado no
    /// fim. Desligar o que não está ligado não é erro.
    #[test]
    fn the_start_with_login_is_saved_read_back_and_removed() {
        let id = format!("dev.aylis.login-test-{}", std::process::id());
        set_app_identity(&id, "Teste do login");
        assert!(!start_with_login(), "nasce desligado");
        set_start_with_login(true).unwrap();
        assert!(start_with_login(), "ligado aponta para este exe");
        set_start_with_login(true).unwrap();
        set_start_with_login(false).unwrap();
        assert!(!start_with_login(), "desligado");
        set_start_with_login(false).unwrap();
    }

    #[test]
    fn the_identity_keeps_only_safe_characters() {
        let id = clean_id("  ../meu app/../x\\y  ");
        assert!(id.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_')), "{id}");
        assert!(!id.starts_with('.'), "{id}");
        assert_eq!(clean_id("dev.aylis.app"), "dev.aylis.app");
    }
}
