//! O runtime do Visual C++ (o `VCRUNTIME140.dll`, o `VCRUNTIME140_1.dll` e o `MSVCP140.dll`)
//! que os executáveis do Rust no Windows (e o Skia, em C++) carregam. Ele vem com o Visual
//! Studio, não com o Windows: numa VM limpa com o Windows 11, o `aylis-tool` e a IDE do
//! instalador nem abriam (`0xC0000135`, DLL não encontrada; 20/11). A Microsoft permite levar
//! essas DLLs ao lado do executável ("app-local"; são os arquivos redistribuíveis do Visual
//! C++), então o instalador da IDE e o publicar as copiam junto.

use std::path::{Path, PathBuf};

/// As DLLs, na ordem em que entram.
pub const DLLS: [&str; 3] = ["vcruntime140.dll", "vcruntime140_1.dll", "msvcp140.dll"];

/// As DLLs desta máquina: as da pasta redistribuível do Visual Studio (`VC\Redist\MSVC\<versão>
/// \x64\Microsoft.VC14x.CRT`, a mais nova) ou, sem ela, as do `System32` (o redistribuível
/// instalado). Vazio fora do Windows ou se faltar alguma.
pub fn dlls() -> Vec<PathBuf> {
    if !cfg!(windows) {
        return Vec::new();
    }
    let redist = visual_studio().and_then(|vs| newest_crt(&vs.join("VC").join("Redist").join("MSVC")));
    let system = std::env::var_os("SystemRoot").map(|r| PathBuf::from(r).join("System32"));
    [redist, system].into_iter().flatten().map(|dir| DLLS.iter().map(|d| dir.join(d)).collect::<Vec<_>>()).find(|all| all.iter().all(|p| p.is_file())).unwrap_or_default()
}

/// A instalação mais nova do Visual Studio com o C++ (pelo `vswhere`).
fn visual_studio() -> Option<PathBuf> {
    let vswhere = PathBuf::from(std::env::var_os("ProgramFiles(x86)")?).join("Microsoft Visual Studio/Installer/vswhere.exe");
    let mut command = std::process::Command::new(vswhere);
    command.args(["-latest", "-products", "*", "-property", "installationPath"]);
    let out = crate::env::output_within(crate::process::quiet(&mut command), crate::env::PROCESS_LIMIT).ok()?;
    let dir = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!dir.is_empty()).then(|| PathBuf::from(dir))
}

/// A pasta `x64\Microsoft.VC14x.CRT` da versão mais nova em `msvc` (as pastas com número de
/// versão: `14.44.35112`; a `v143` é dos módulos de mesclagem).
fn newest_crt(msvc: &Path) -> Option<PathBuf> {
    let mut versions: Vec<(Vec<u32>, PathBuf)> = std::fs::read_dir(msvc)
        .ok()?
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            let parts: Option<Vec<u32>> = name.split('.').map(|p| p.parse().ok()).collect();
            parts.filter(|p| p.len() >= 2).map(|p| (p, e.path()))
        })
        .collect();
    versions.sort();
    versions.into_iter().rev().find_map(|(_, dir)| {
        std::fs::read_dir(dir.join("x64")).ok()?.flatten().map(|e| e.path()).find(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("Microsoft.VC")) && p.join(DLLS[0]).is_file())
    })
}

/// Copia as DLLs para `dir` (ao lado do executável publicado); devolve as copiadas. Sem elas na
/// máquina, um aviso: o app só abre onde o redistribuível do Visual C++ estiver instalado.
pub fn copy_beside(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut copied = Vec::new();
    for from in dlls() {
        let dest = dir.join(from.file_name().unwrap_or_default());
        std::fs::copy(&from, &dest).map_err(|e| format!("{}: {e}", from.display()))?;
        copied.push(dest);
    }
    Ok(copied)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A versão mais nova com número (a `v143` fica de fora) e só a pasta que tem as DLLs.
    #[test]
    fn the_newest_redist_folder_wins() {
        let root = std::env::temp_dir().join(format!("aylis_vcruntime_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (version, crt) in [("14.38.33135", "Microsoft.VC143.CRT"), ("14.44.35112", "Microsoft.VC143.CRT"), ("v143", "Microsoft.VC143.CRT")] {
            let dir = root.join(version).join("x64").join(crt);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join(DLLS[0]), b"x").unwrap();
        }
        std::fs::create_dir_all(root.join("14.50.1").join("x64").join("Microsoft.VC145.CRT")).unwrap(); // sem as DLLs
        assert_eq!(newest_crt(&root), Some(root.join("14.44.35112").join("x64").join("Microsoft.VC143.CRT")));
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Nesta máquina (que compila o Aylis no Windows), as três existem.
    #[test]
    #[cfg(windows)]
    fn this_machine_has_the_three() {
        let found = dlls();
        assert_eq!(found.len(), 3, "{found:?}");
    }
}
