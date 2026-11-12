//! # aylis-tools
//!
//! O "Executar" e o "Publicar" do aylis, sem interface (a IDE e o `aylis-tool` usam):
//!
//! - [`env`]: verifica o ambiente de cada plataforma (Rust, linker, Python, Android SDK e
//!   NDK, Java, wasm-bindgen, Xcode) e sugere o comando que corrige cada falta;
//! - [`run`]: compila e executa um projeto (desktop, Android, navegador), com os erros do
//!   compilador estruturados (arquivo, linha, coluna) para a lista clicável;
//! - [`publish`]: gera o aplicativo final (.exe, APK, pacote web, .app).
//!
//! Tudo roda numa thread de trabalho: o andamento chega como [`Event`]s, e um [`Control`]
//! interrompe (mata o processo em andamento).
//!
//! ```text
//! aylis-tool check                        verifica o ambiente
//! aylis-tool run Vendas/Vendas.aylisproj   compila e executa
//! aylis-tool publish Vendas/Vendas.aylisproj --target android
//! ```

#[cfg(not(target_arch = "wasm32"))]
pub mod agent;
pub mod cputime;
pub mod dap;
pub mod debug;
pub mod devkeys;
pub mod downloads;
pub mod installer;
pub mod instances;
pub mod env;
pub mod hostlang;
pub mod icons;
pub mod ide_package;
pub mod setup;
pub mod ibge;
pub mod translations;
pub mod jdb_dap;
pub mod lsp;
pub mod mcp;
pub mod packages;
pub mod build;
pub mod build_index;
pub mod cargo_config;
pub mod cleanup;
pub mod process;
pub mod project_tests;
pub mod publish;
pub mod resources;
pub mod testdocs;
pub mod emoji;
pub mod bench;
pub mod report;
pub mod run;
pub mod serve;
pub mod settings;
pub mod shell;
pub mod touched;
pub mod trust;
pub mod updates;
pub mod wsl;

use std::path::{Path, PathBuf};

pub use process::{Control, Event, LineKind, Sink};

/// Plataforma de destino.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Target {
    /// O sistema onde a IDE está (Windows, Linux ou macOS).
    Desktop,
    Android,
    Web,
    Ios,
    /// Linux a partir do Windows, pelo WSL (a janela aparece pelo WSLg).
    Linux,
}

impl Target {
    pub const ALL: [Target; 5] = [Target::Desktop, Target::Linux, Target::Android, Target::Web, Target::Ios];

    /// As saídas que fazem sentido para a linguagem do projeto, neste sistema:
    /// - em Rust, o desktop, o Linux pelo WSL (num Windows), o Android, a web e o iOS (só num
    ///   Mac, onde o Xcode compila);
    /// - nas outras linguagens, só o desktop (o código delas roda no computador; no celular
    ///   e no navegador é a casca da Publicação, ligada a um servidor).
    pub fn available(language: aylis_project::Language) -> Vec<Target> {
        if language != aylis_project::Language::Rust {
            return vec![Target::Desktop];
        }
        Target::ALL
            .into_iter()
            .filter(|t| match t {
                Target::Linux => cfg!(windows),
                Target::Ios => cfg!(target_os = "macos"),
                _ => true,
            })
            .collect()
    }

    pub fn name(self) -> &'static str {
        match self {
            Target::Desktop => desktop_name(),
            Target::Linux => "Linux (WSL)",
            Target::Android => "Android",
            Target::Web => "Navegador (web)",
            Target::Ios => "iOS",
        }
    }

    /// Nome curto (pastas, linha de comando).
    pub fn id(self) -> &'static str {
        match self {
            Target::Desktop => {
                if cfg!(windows) {
                    "windows"
                } else if cfg!(target_os = "macos") {
                    "macos"
                } else {
                    "linux"
                }
            }
            Target::Android => "android",
            Target::Web => "web",
            Target::Ios => "ios",
            Target::Linux => "linux",
        }
    }

    pub fn from_id(id: &str) -> Option<Target> {
        match id.to_lowercase().as_str() {
            "linux" | "wsl" if cfg!(windows) => Some(Target::Linux),
            "desktop" | "windows" | "linux" | "macos" => Some(Target::Desktop),
            "android" | "apk" => Some(Target::Android),
            "web" | "wasm" | "navegador" => Some(Target::Web),
            "ios" | "iphone" => Some(Target::Ios),
            _ => None,
        }
    }
}

fn desktop_name() -> &'static str {
    if cfg!(windows) {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else {
        "Linux"
    }
}

/// Pasta do aylis (onde estão os crates e os resources).
pub fn aylis_root() -> PathBuf {
    PathBuf::from(aylis_project::codegen::aylis_root())
}

/// Pasta `target/` compartilhada: o que o aylis já compilou é reaproveitado por todos os
/// projetos (a primeira compilação de um projeto fica bem mais rápida).
///
/// A escolhida nas Opções da IDE (`settings::target_dir`) vale sempre. Sem ela: rodando do
/// repositório, é o `target/` dele; instalada, a IDE não grava na pasta da instalação e usa
/// `%LOCALAPPDATA%\aylis\target` (`~/.cache/aylis/target` fora do Windows).
pub fn shared_target_dir() -> PathBuf {
    if let Some(dir) = settings::target_dir() {
        return dir;
    }
    default_target_dir()
}

/// A pasta de compilação sem a escolha das Opções (o que a tela mostra como automático): a
/// mesma rodando do repositório ou instalada, porque o `config.toml` global do cargo
/// (`cargo_config`) leva todo o Rust da máquina para ela, o repositório do Aylis inclusive.
pub fn default_target_dir() -> PathBuf {
    let root = aylis_root();
    let base = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
    } else {
        std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
    };
    base.map(|b| b.join("aylis").join("target")).unwrap_or_else(|| root.join("target"))
}

/// Onde o cargo compila o workspace de `dir` (o `CARGO_TARGET_DIR`, o `config.toml` do projeto
/// ou o global), pelo `cargo metadata`; sem o cargo, `<dir>/target`. Para o que procura os
/// executáveis compilados do repositório (o instalador, o bench), que não ficam mais
/// necessariamente no `target/` dele.
pub fn cargo_target_dir(dir: &Path) -> PathBuf {
    let out = process::quiet(std::process::Command::new("cargo").args(["metadata", "--no-deps", "--format-version", "1"]).current_dir(dir)).output();
    out.ok()
        .filter(|o| o.status.success())
        .and_then(|o| serde_json::from_slice::<serde_json::Value>(&o.stdout).ok())
        .and_then(|v| v["target_directory"].as_str().map(PathBuf::from))
        .unwrap_or_else(|| dir.join("target"))
}

/// Onde o executável `name` está: no `PATH` (com `.exe`/`.cmd`/`.bat` no Windows).
pub fn which(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let exts: &[&str] = if cfg!(windows) { &[".exe", ".cmd", ".bat", ""] } else { &[""] };
    std::env::split_paths(&path).find_map(|dir| {
        exts.iter().map(|e| dir.join(format!("{name}{e}"))).find(|p| p.is_file())
    })
}

/// Um local no código (para a lista de erros clicável).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub file: PathBuf,
    pub line: u32,
    pub column: u32,
}

/// Acha um local numa linha de saída: `src/forms/x.rs:12:5` (Rust, inclusive em
/// `panicked at`), `File "x.py", line 12` (Python). Caminhos relativos ficam relativos a
/// `base`. Só vale se o arquivo existir.
pub fn location_in(line: &str, base: &Path) -> Option<Location> {
    let resolve = |file: &str| {
        let path = PathBuf::from(file.trim());
        let path = if path.is_absolute() { path } else { base.join(path) };
        path.is_file().then_some(path)
    };
    // Python: File "c:\proj\src\forms\principal.py", line 12, in ...
    if let Some(start) = line.find("File \"") {
        let rest = &line[start + 6..];
        let end = rest.find('"')?;
        let file = resolve(&rest[..end])?;
        let after = &rest[end..];
        let n = after.find("line ").map(|i| &after[i + 5..])?;
        let digits: String = n.chars().take_while(char::is_ascii_digit).collect();
        return Some(Location { file, line: digits.parse().ok()?, column: 1 });
    }
    // Rust: ...arquivo.rs:12:5
    for word in line.split(|c: char| c.is_whitespace() || c == '(' || c == ')' || c == ',') {
        let word = word.trim_end_matches(':');
        let Some(idx) = word.find(".rs:") else { continue };
        let file = &word[..idx + 3];
        let mut nums = word[idx + 4..].split(':');
        let line_no = nums.next().and_then(|n| n.parse().ok());
        let col = nums.next().and_then(|n| n.parse().ok()).unwrap_or(1);
        if let (Some(line_no), Some(file)) = (line_no, resolve(file)) {
            return Some(Location { file, line: line_no, column: col });
        }
    }
    None
}

/// Nome de arquivo seguro para o nome do aplicativo (`Vendas: 2024` → `Vendas 2024`).
pub fn file_name(name: &str) -> String {
    let clean: String = name.chars().filter(|c| !r#"<>:"/\|?*"#.contains(*c) && !c.is_control()).collect();
    let clean = clean.trim().trim_end_matches('.').to_string();
    if clean.is_empty() { "app".into() } else { clean }
}

/// Copia uma pasta inteira (cria o destino).
pub fn copy_dir(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    for entry in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let dest = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &dest)?;
        } else {
            std::fs::copy(entry.path(), &dest).map_err(|e| format!("{}: {e}", dest.display()))?;
        }
    }
    Ok(())
}

/// Bytes aleatórios do sistema (as chaves, os tokens e os segredos). No navegador (a IDE
/// lite), ainda não: ela não cria chaves nem abre servidores.
pub fn random_bytes(out: &mut [u8]) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    return getrandom::fill(out).map_err(|e| format!("sem gerador aleatório do sistema: {e}"));
    #[cfg(target_arch = "wasm32")]
    {
        let _ = out;
        Err("sem gerador aleatório no navegador".into())
    }
}
