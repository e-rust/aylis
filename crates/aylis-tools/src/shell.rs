//! Abrir arquivos, pastas e endereços com o programa padrão, e o VS Code, sem passar por um
//! interpretador de comandos: com `cmd /C`, um caminho sem espaços com `&` (`a&calc.rs`)
//! virava comando. Aqui o alvo vai como texto para o sistema.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Abre `target` (arquivo, pasta ou endereço) com o programa padrão do sistema, como o duplo
/// clique: `ShellExecuteW` no Windows, `open` no macOS, `xdg-open` no Linux.
pub fn open(target: &str) -> Result<(), String> {
    if target.is_empty() {
        return Err("nada para abrir".into());
    }
    #[cfg(windows)]
    {
        win::shell_execute(target)
    }
    #[cfg(not(windows))]
    {
        // Um alvo que começa com `-` seria lido como opção.
        let target = if target.starts_with('-') { format!("./{target}") } else { target.to_string() };
        let program = if cfg!(target_os = "macos") { "open" } else { "xdg-open" };
        let status = crate::process::quiet(Command::new(program).arg(&target)).status().map_err(|e| format!("{program}: {e}"))?;
        if status.success() { Ok(()) } else { Err(format!("{program} saiu com {status}")) }
    }
}

/// O executável de linha de comando do VS Code: no PATH ou no lugar padrão da instalação
/// de cada sistema.
pub fn vscode() -> Option<PathBuf> {
    let name = if cfg!(windows) { "code.cmd" } else { "code" };
    let in_path = std::env::var_os("PATH").and_then(|paths| std::env::split_paths(&paths).map(|d| d.join(name)).find(|p| p.is_file()));
    in_path.or_else(|| default_vscode_locations().into_iter().find(|p| p.is_file()))
}

fn default_vscode_locations() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if cfg!(windows) {
        for var in ["LOCALAPPDATA", "ProgramFiles"] {
            if let Some(base) = std::env::var_os(var) {
                let base = PathBuf::from(base);
                let base = if var == "LOCALAPPDATA" { base.join("Programs") } else { base };
                out.push(base.join("Microsoft VS Code").join("bin").join("code.cmd"));
            }
        }
    } else if cfg!(target_os = "macos") {
        out.push(PathBuf::from("/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code"));
        if let Some(home) = std::env::var_os("HOME") {
            out.push(Path::new(&home).join("Applications/Visual Studio Code.app/Contents/Resources/app/bin/code"));
        }
    } else {
        out.extend(["/usr/bin/code", "/usr/local/bin/code", "/snap/bin/code"].map(PathBuf::from));
    }
    out
}

/// Roda o VS Code com `args`, chamado direto (no Windows, o `code.cmd`: a biblioteca padrão do
/// Rust escapa os argumentos de um `.cmd`, e recusa os que não dá para escapar com segurança).
pub fn run_vscode<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> Result<(), String> {
    let program = vscode().ok_or("VS Code não encontrado (instale e marque \"Adicionar ao PATH\", ou informe o caminho nas Opções da IDE)")?;
    let status = crate::process::quiet(Command::new(&program).args(args)).status().map_err(|e| format!("não consegui abrir o VS Code: {e}"))?;
    if status.success() { Ok(()) } else { Err(format!("o VS Code saiu com {status}")) }
}

#[cfg(windows)]
mod win {
    use std::os::windows::ffi::OsStrExt;

    #[link(name = "shell32")]
    unsafe extern "system" {
        fn ShellExecuteW(hwnd: isize, operation: *const u16, file: *const u16, parameters: *const u16, directory: *const u16, show: i32) -> isize;
    }

    const SW_SHOWNORMAL: i32 = 1;

    fn wide(text: &str) -> Vec<u16> {
        std::ffi::OsStr::new(text).encode_wide().chain(Some(0)).collect()
    }

    pub fn shell_execute(target: &str) -> Result<(), String> {
        let (verb, file) = (wide("open"), wide(target));
        // SAFETY: as strings terminam em zero e vivem durante a chamada.
        let code = unsafe { ShellExecuteW(0, verb.as_ptr(), file.as_ptr(), std::ptr::null(), std::ptr::null(), SW_SHOWNORMAL) };
        // Acima de 32 é sucesso (a documentação do ShellExecute).
        if code > 32 { Ok(()) } else { Err(format!("o Windows não conseguiu abrir '{target}' (código {code})")) }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn nothing_to_open_is_an_error_and_vscode_has_default_places() {
        assert!(super::open("").is_err());
        assert!(!super::default_vscode_locations().is_empty());
        // Achado ou não, a busca não roda nada.
        let _ = super::vscode();
    }
}
