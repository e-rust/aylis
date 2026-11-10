//! Configurar uma linguagem sem passo manual: instala o compilador (ou o interpretador), o
//! depurador e o servidor de linguagem, e confere de novo até não sobrar nada que dê para
//! instalar. É o que o assistente "Configurar linguagens" da IDE e o `aylis-tool setup` usam.
//!
//! No Windows tudo vem pelo winget, sem perguntas (`--silent` e os termos aceitos); o
//! Windows ainda pede a permissão de administrador (UAC) nos que instalam para a máquina.
//! No macOS, pelo Homebrew; no Linux, pelo apt (o `pkexec` pede a senha numa janela).
//!
//! ```text
//! aylis-tool setup go csharp        configura essas
//! aylis-tool setup --all            todas as 8
//! ```

use std::path::{Path, PathBuf};
use std::process::Command;

use aylis_project::Language;

use crate::env::{self, Check, Fix, Status};
use crate::process::{Control, Event, LineKind};
use crate::{Target, which};

/// Os argumentos do winget que tiram as perguntas.
const WINGET_QUIET: [&str; 5] = ["--silent", "--accept-package-agreements", "--accept-source-agreements", "--disable-interactivity", "-e"];

/// Instala um pacote do winget (o id exato) sem perguntas.
fn winget(description: &str, id: &str, extra: &[&str]) -> Fix {
    let mut args = vec!["winget", "install", "--id", id];
    args.extend(WINGET_QUIET);
    args.extend(extra);
    Fix { description: format!("{description} (winget, {id})"), command: args.iter().map(|s| s.to_string()).collect(), interactive: false, fetch: None, env: Vec::new() }
}

fn shell(description: &str, script: &str) -> Fix {
    let (program, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };
    Fix { description: description.to_string(), command: vec![program.into(), flag.into(), script.into()], interactive: false, fetch: None, env: Vec::new() }
}

/// O apt pelo `pkexec` (a senha numa janela do sistema, sem terminal).
fn apt(description: &str, packages: &str) -> Fix {
    shell(description, &format!("pkexec sh -c 'apt-get update && apt-get install -y {packages}'"))
}

fn brew(description: &str, packages: &str) -> Fix {
    shell(description, &format!("brew install {packages}"))
}

/// O compilador ou o interpretador da linguagem (o que roda e compila os projetos dela).
pub fn install_toolchain(language: Language) -> Option<Fix> {
    if cfg!(windows) {
        Some(match language {
            // O instalador do rustup não traz o toolchain: o `default stable` baixa.
            Language::Rust => {
                let rustup = home().map(|h| h.join(".cargo/bin/rustup.exe").display().to_string()).unwrap_or_else(|| "rustup".into());
                let quiet = WINGET_QUIET.join(" ");
                shell(
                    "Instala o Rust (winget, Rustlang.Rustup) e o toolchain stable",
                    &format!("winget install --id Rustlang.Rustup {quiet} & \"{rustup}\" default stable"),
                )
            }
            Language::Python => winget(
                "Instala o Python 3.12 para o usuário, com o pip e no PATH",
                "Python.Python.3.12",
                &["--scope", "user", "--override", "/quiet InstallAllUsers=0 PrependPath=1 Include_launcher=1 Include_pip=1"],
            ),
            Language::TypeScript => winget("Instala o Node LTS (roda o TypeScript e o depurador)", "OpenJS.NodeJS.LTS", &[]),
            Language::CSharp => winget("Instala o .NET SDK 8", "Microsoft.DotNet.SDK.8", &[]),
            Language::Java => winget("Instala o JDK 17 (Eclipse Temurin)", "EclipseAdoptium.Temurin.17.JDK", &[]),
            Language::Go => winget("Instala o Go", "GoLang.Go", &[]),
            Language::Cpp => msvc(),
            // O Lazarus traz o Free Pascal de 64 bits e o lazbuild (que compila o pasls).
            Language::Pascal => winget("Instala o Lazarus (com o Free Pascal de 64 bits)", "Lazarus.Lazarus", &[]),
        })
    } else if cfg!(target_os = "macos") {
        Some(match language {
            Language::Rust => shell("Instala o Rust (rustup)", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"),
            Language::Python => brew("Instala o Python 3 (Homebrew)", "python"),
            Language::TypeScript => brew("Instala o Node (Homebrew)", "node"),
            Language::CSharp => brew("Instala o .NET SDK 8 (Homebrew)", "--cask dotnet-sdk@8"),
            Language::Java => brew("Instala o JDK 17 (Homebrew)", "openjdk@17"),
            Language::Go => brew("Instala o Go (Homebrew)", "go"),
            Language::Cpp => Fix { description: "Instala as ferramentas do Xcode (clang)".into(), command: vec!["xcode-select".into(), "--install".into()], interactive: true, fetch: None, env: Vec::new() },
            Language::Pascal => brew("Instala o Free Pascal e o Lazarus (Homebrew)", "fpc && brew install --cask lazarus"),
        })
    } else {
        Some(match language {
            Language::Rust => shell("Instala o Rust (rustup)", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"),
            Language::Python => apt("Instala o Python 3 (apt)", "python3 python3-pip python3-venv"),
            Language::TypeScript => apt("Instala o Node e o npm (apt)", "nodejs npm"),
            Language::CSharp => apt("Instala o .NET SDK 8 (apt)", "dotnet-sdk-8.0"),
            Language::Java => apt("Instala o JDK 17 (apt)", "openjdk-17-jdk"),
            Language::Go => apt("Instala o Go (apt)", "golang-go"),
            Language::Cpp => apt("Instala o g++ (apt)", "build-essential"),
            Language::Pascal => apt("Instala o Free Pascal e o Lazarus (apt)", "fpc lazarus"),
        })
    }
}

/// As Build Tools do Visual Studio com o C++ (o compilador do C++ e o linker do Rust).
pub fn msvc() -> Fix {
    winget(
        "Instala as Build Tools do Visual Studio com C++ (pode levar vários minutos)",
        "Microsoft.VisualStudio.2022.BuildTools",
        &["--override", "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"],
    )
}

/// O git (o pasls do Pascal vem do GitHub): o do PATH ou o que o winget instala.
pub fn git() -> String {
    which("git").map(|p| p.display().to_string()).unwrap_or_else(|| {
        let program_files = std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".into());
        format!("{program_files}\\Git\\cmd\\git.exe")
    })
}

fn home() -> Option<PathBuf> {
    std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from)
}

/// Põe no PATH deste processo o que os instaladores acabaram de acrescentar: no Windows, o
/// PATH do sistema e o do usuário guardados no registro; em todos, as pastas de sempre do
/// rustup, do Go e do .NET. Sem isso a IDE aberta só veria o que instalou depois de
/// reiniciar.
pub fn refresh_path() {
    let current = std::env::var_os("PATH").unwrap_or_default();
    let mut dirs: Vec<PathBuf> = std::env::split_paths(&current).collect();
    let mut add = |d: PathBuf| {
        let key = |p: &Path| p.display().to_string().trim_end_matches(['\\', '/']).to_lowercase();
        if !d.as_os_str().is_empty() && d.is_dir() && !dirs.iter().any(|x| key(x) == key(&d)) {
            dirs.push(d);
        }
    };
    if cfg!(windows) {
        for key in ["HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", "HKCU\\Environment"] {
            for d in registry_path(key) {
                add(d);
            }
        }
    }
    if let Some(h) = home() {
        add(h.join(".cargo").join("bin"));
        add(h.join(".dotnet"));
        add(h.join("go").join("bin"));
    }
    if !cfg!(windows) {
        add(PathBuf::from("/usr/local/go/bin"));
        add(PathBuf::from("/opt/homebrew/bin"));
    }
    if let Ok(joined) = std::env::join_paths(&dirs) {
        if joined != current {
            // SAFETY: só muda o PATH (os outros leem com `var_os`, que copia); a IDE chama
            // antes de verificar ou instalar, e nenhuma biblioteca C lê o PATH ao mesmo tempo.
            unsafe { std::env::set_var("PATH", joined) };
        }
    }
}

/// As pastas do `Path` numa chave do registro, com as `%VARIAVEIS%` trocadas.
fn registry_path(key: &str) -> Vec<PathBuf> {
    let Ok(out) = crate::process::quiet(&mut Command::new("reg")).args(["query", key, "/v", "Path"]).output() else { return Vec::new() };
    let text = String::from_utf8_lossy(&out.stdout);
    let Some(value) = text.lines().find_map(|l| {
        let l = l.trim();
        let rest = l.strip_prefix("Path").or_else(|| l.strip_prefix("PATH"))?;
        let rest = rest.trim_start();
        let rest = rest.strip_prefix("REG_EXPAND_SZ").or_else(|| rest.strip_prefix("REG_SZ"))?;
        Some(rest.trim().to_string())
    }) else {
        return Vec::new();
    };
    value.split(';').filter(|s| !s.trim().is_empty()).map(|s| PathBuf::from(expand(s.trim()))).collect()
}

/// `%USERPROFILE%\.cargo\bin` → `C:\Users\...\.cargo\bin`.
fn expand(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find('%') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        match after.find('%') {
            Some(end) => {
                let name = &after[..end];
                out.push_str(&std::env::var(name).unwrap_or_else(|_| format!("%{name}%")));
                rest = &after[end + 1..];
            }
            None => {
                out.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

/// A linguagem já roda projetos (o compilador ou o interpretador está lá)? Rápido: serve
/// para o assistente marcar o que já está instalado.
pub fn language_ready(language: Language) -> bool {
    match language {
        Language::Rust => which("cargo").is_some(),
        Language::Python => env::python().is_some_and(|(_, (major, minor, _))| major == 3 && minor >= 8),
        other => crate::hostlang::find(other).is_ok(),
    }
}

/// O que entra na configuração: o que falta para executar e depurar no desktop e o servidor
/// de linguagem (o WiX é só para o instalador: fica para o Verificar ambiente).
fn wanted(check: &Check) -> bool {
    check.target == Target::Desktop && check.status != Status::Ok && check.fix.is_some() && !check.name.starts_with("WiX")
}

/// O que ainda falta para a linguagem ficar completa no desktop (o depurador, o servidor de
/// linguagem...), pelos nomes da verificação.
pub fn missing(language: Language, aylis_host: Option<&Path>) -> Vec<String> {
    env::check(language, &[Target::Desktop], aylis_host)
        .into_iter()
        .filter(|c| c.status != Status::Ok && !c.name.starts_with("WiX"))
        .map(|c| c.name.split(" (").next().unwrap_or(&c.name).to_string())
        .collect()
}

/// Configura a linguagem: roda as correções do que falta (primeiro o compilador; com ele, o
/// depurador e o servidor de linguagem aparecem com o que os instala), refaz o PATH e
/// confere de novo, até nada mais ter correção. Devolve a verificação final; o que ainda
/// faltar vem nela.
pub fn setup(language: Language, aylis_host: Option<&Path>, control: &Control, log: &mut dyn FnMut(Event)) -> Vec<Check> {
    let mut tried: Vec<String> = Vec::new();
    loop {
        refresh_path();
        let checks = env::check(language, &[Target::Desktop], aylis_host);
        let next = checks.iter().filter(|c| wanted(c)).find(|c| !tried.contains(&c.fix.as_ref().map(Fix::command_line).unwrap_or_default()));
        let Some(check) = next else { return checks };
        let fix = check.fix.clone().unwrap_or_else(|| unreachable!("filtrado por wanted"));
        tried.push(fix.command_line());
        if control.is_cancelled() {
            return checks;
        }
        log(Event::Step(format!("{}: {}", language.name(), fix.description)));
        log(Event::Line(format!("> {}", fix.command_line()), LineKind::Tool));
        // Um passo que falha não para os outros (o winget sai com erro quando já está
        // instalado, por exemplo): a verificação seguinte diz se resolveu.
        if let Err(e) = fix.run(control, &mut |e| log(e)) {
            log(Event::Line(format!("⚠ {e}"), LineKind::Tool));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_environment_variables() {
        let home = std::env::var(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).unwrap();
        let name = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        assert_eq!(expand(&format!(r"%{name}%\.cargo\bin")), format!(r"{home}\.cargo\bin"));
        assert_eq!(expand(r"%AYLIS_NAO_EXISTE%\x"), r"%AYLIS_NAO_EXISTE%\x", "a desconhecida fica como está");
        assert_eq!(expand("sem variavel"), "sem variavel");
        assert_eq!(expand("50%"), "50%");
    }

    #[test]
    #[cfg(windows)]
    fn reads_the_system_path_from_the_registry() {
        let dirs = registry_path(r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment");
        assert!(dirs.iter().any(|d| d.display().to_string().to_lowercase().contains("system32")), "{dirs:?}");
        // Sem perder o que já estava no PATH.
        let before: Vec<PathBuf> = std::env::split_paths(&std::env::var_os("PATH").unwrap()).collect();
        refresh_path();
        let after: Vec<PathBuf> = std::env::split_paths(&std::env::var_os("PATH").unwrap()).collect();
        assert!(before.iter().all(|d| after.contains(d)));
    }
}
