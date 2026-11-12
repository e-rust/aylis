//! Verificação do ambiente: o que cada plataforma precisa para compilar e executar, e o
//! comando que corrige cada falta (a IDE pergunta antes de rodar).

use std::path::{Path, PathBuf};
use std::process::Command;

use aylis_project::Language;

use crate::process::quiet;
use crate::{Target, which};

/// Versão mínima do Rust: o `rust-version` do workspace do Aylis (estava 1.86 com o
/// workspace em 1.88, e o 1.87 passava na conferência e falhava no cargo). O teste
/// `the_minimum_rust_is_the_workspace_one` confere.
pub const MIN_RUST: (u32, u32) = (1, 88);

#[cfg(test)]
mod min_rust_tests {
    #[test]
    fn the_minimum_rust_is_the_workspace_one() {
        let manifest = include_str!("../../../Cargo.toml");
        let line = manifest.lines().find(|l| l.trim_start().starts_with("rust-version")).expect("rust-version no workspace");
        let version = line.split('"').nth(1).unwrap();
        assert_eq!(version, format!("{}.{}", super::MIN_RUST.0, super::MIN_RUST.1));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    /// Funciona, mas falta algo opcional (um aparelho conectado, por exemplo).
    Warning,
    Missing,
}

/// Um comando que corrige uma falta.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix {
    /// O que ele faz, para mostrar antes de perguntar.
    pub description: String,
    /// Programa e argumentos.
    pub command: Vec<String>,
    /// Abre uma janela ou instalador (a IDE não espera a saída; o usuário acompanha).
    pub interactive: bool,
    /// Em vez de rodar o comando: baixar a ferramenta fixada com este nome, conferir o
    /// SHA-256 e instalar ([`crate::downloads::fetch`]; o comando é o `aylis-tool fetch`
    /// equivalente, para mostrar).
    pub fetch: Option<String>,
    /// Variáveis de ambiente do comando (o `GOBIN` do `go install`).
    pub env: Vec<(String, String)>,
}

impl Fix {
    fn new(description: impl Into<String>, command: &[&str]) -> Self {
        Self { description: description.into(), command: command.iter().map(|s| s.to_string()).collect(), interactive: false, fetch: None, env: Vec::new() }
    }

    /// Baixar uma ferramenta fixada em `resources/tools.lock.json` (`what`: "o CodeLLDB").
    fn download(name: &str, what: &str) -> Option<Self> {
        let a = crate::downloads::archive(name)?;
        let mut fix = Fix::new(
            format!("Baixa {what} {} ({}, ~{} MB), confere o SHA-256 e instala na pasta de ferramentas do Aylis", a.version, a.file, a.megabytes()),
            &["aylis-tool", "fetch", name],
        );
        fix.fetch = Some(name.to_string());
        Some(fix)
    }

    /// Roda a correção (não interativa) e manda a saída para `sink`.
    pub fn run(&self, control: &crate::Control, sink: crate::Sink) -> Result<(), String> {
        if let Some(name) = &self.fetch {
            control.check()?;
            crate::downloads::fetch(name, &mut |line| sink(crate::Event::Line(line, crate::LineKind::Tool)))?;
            return Ok(());
        }
        let (program, args) = self.command.split_first().ok_or("comando vazio")?;
        let mut command = Command::new(program);
        command.args(args).envs(self.env.iter().map(|(k, v)| (k, v)));
        crate::process::run_tool(&mut command, control, sink, crate::LineKind::Tool)
    }

    fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    /// O comando como se digita num terminal.
    pub fn command_line(&self) -> String {
        self.command.iter().map(|a| if a.contains(' ') { format!("\"{a}\"") } else { a.clone() }).collect::<Vec<_>>().join(" ")
    }
}

/// Um item verificado.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    pub target: Target,
    pub name: String,
    pub status: Status,
    /// Versão encontrada, ou o que falta.
    pub detail: String,
    pub fix: Option<Fix>,
    /// Instrução quando não há comando automático (link, passo manual).
    pub help: Option<String>,
}

impl Check {
    fn ok(target: Target, name: &str, detail: impl Into<String>) -> Self {
        Self { target, name: name.into(), status: Status::Ok, detail: detail.into(), fix: None, help: None }
    }

    fn missing(target: Target, name: &str, detail: impl Into<String>) -> Self {
        Self { target, name: name.into(), status: Status::Missing, detail: detail.into(), fix: None, help: None }
    }

    fn warning(target: Target, name: &str, detail: impl Into<String>) -> Self {
        Self { target, name: name.into(), status: Status::Warning, detail: detail.into(), fix: None, help: None }
    }

    fn fix(mut self, fix: Fix) -> Self {
        self.fix = Some(fix);
        self
    }

    fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }
}

/// Tudo que `target` precisa está presente (avisos não impedem).
pub fn ready(checks: &[Check], target: Target) -> bool {
    checks.iter().filter(|c| c.target == target).all(|c| c.status != Status::Missing)
}

/// O que falta para `target` (para a mensagem antes de executar).
pub fn missing(checks: &[Check], target: Target) -> Vec<&Check> {
    checks.iter().filter(|c| c.target == target && c.status == Status::Missing).collect()
}

/// Verifica o ambiente para as plataformas pedidas. `aylis_host` é o `aylis-host` que roda
/// projetos Python (ao lado da IDE). Demora alguns segundos (roda as ferramentas): chame
/// fora da thread da interface.
pub fn check(language: Language, targets: &[Target], aylis_host: Option<&Path>) -> Vec<Check> {
    // O que acabou de ser instalado (pela IDE ou fora dela) entra no PATH sem reiniciar.
    crate::setup::refresh_path();
    let mut out = Vec::new();
    let rust = rust_checks(language == Language::Rust || targets.iter().any(|t| *t != Target::Desktop));
    for &target in targets {
        match target {
            Target::Desktop => desktop(&mut out, language, &rust, aylis_host),
            Target::Android => android(&mut out, language, &rust),
            Target::Web => web(&mut out, language, &rust),
            Target::Ios => ios(&mut out, language, &rust),
            Target::Linux => linux_wsl(&mut out, language),
        }
    }
    out
}

/// O CodeLLDB (o pacote da extensão do VS Code, ~50 MB, do GitHub), na versão fixada,
/// extraído na pasta de ferramentas do aylis, onde a IDE procura.
fn install_codelldb() -> Option<Fix> {
    Fix::download("codelldb", "o CodeLLDB")
}

/// O que se sabe do Rust (verificado uma vez para todas as plataformas).
#[derive(Default)]
struct Rust {
    cargo: Option<String>,
    /// O cargo existe, mas falhou (o toolchain não está instalado, por exemplo): a mensagem.
    cargo_error: Option<String>,
    rustc: Option<(u32, u32, u32)>,
    targets: Vec<String>,
    has_rustup: bool,
}

fn rust_checks(needed: bool) -> Rust {
    if !needed {
        return Rust::default();
    }
    let (cargo, cargo_error) = match run("cargo", &["--version"]) {
        Ok(v) => (Some(v), None),
        Err(e) => (None, e),
    };
    let rustc = output("rustc", &["--version"]).and_then(|v| parse_version(&v));
    let targets = output("rustup", &["target", "list", "--installed"]);
    Rust {
        cargo,
        cargo_error,
        rustc,
        has_rustup: targets.is_some(),
        targets: targets.map(|t| t.lines().map(|l| l.trim().to_string()).collect()).unwrap_or_default(),
    }
}

fn rust_items(out: &mut Vec<Check>, target: Target, rust: &Rust) {
    match &rust.cargo {
        Some(v) => out.push(Check::ok(target, "Rust (cargo)", v.trim())),
        None => {
            // O erro de verdade, quando o cargo existe e falhou (antes era sempre "não
            // encontrado"; o relatório do Tabernáculo, I02).
            let detail = rust.cargo_error.as_deref().and_then(|e| e.lines().map(str::trim).find(|l| !l.is_empty())).unwrap_or("não encontrado");
            let check = Check::missing(target, "Rust (cargo)", detail);
            out.push(match crate::setup::install_toolchain(Language::Rust) {
                Some(fix) => check.fix(fix),
                None => check.help("Instale pelo https://rustup.rs (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)"),
            });
            return;
        }
    }
    match rust.rustc {
        Some((major, minor, patch)) if (major, minor) >= MIN_RUST => {
            out.push(Check::ok(target, "Versão do Rust", format!("{major}.{minor}.{patch}")));
        }
        Some((major, minor, patch)) => out.push(
            Check::missing(target, "Versão do Rust", format!("{major}.{minor}.{patch} (precisa de {}.{} ou mais nova)", MIN_RUST.0, MIN_RUST.1))
                .fix(Fix::new("Atualiza o Rust", &["rustup", "update", "stable"])),
        ),
        None => out.push(Check::missing(target, "Versão do Rust", "rustc não respondeu")),
    }
}

/// Alvos do Rust que faltam (com a correção).
fn rust_targets(out: &mut Vec<Check>, target: Target, rust: &Rust, name: &str, wanted: &[&str]) {
    let missing: Vec<&str> = wanted.iter().copied().filter(|t| !rust.targets.iter().any(|i| i == t)).collect();
    if missing.is_empty() {
        out.push(Check::ok(target, name, wanted.join(", ")));
    } else if !rust.has_rustup {
        out.push(Check::missing(target, name, format!("faltam {}", missing.join(", "))).help("Instale o rustup (https://rustup.rs) para adicionar alvos"));
    } else {
        let mut args = vec!["rustup", "target", "add"];
        args.extend(&missing);
        out.push(
            Check::missing(target, name, format!("faltam {}", missing.join(", ")))
                .fix(Fix::new(format!("Adiciona os alvos {}", missing.join(", ")), &args)),
        );
    }
}

// ---------------------------------------------------------------------------
// Desktop

fn desktop(out: &mut Vec<Check>, language: Language, rust: &Rust, aylis_host: Option<&Path>) {
    let t = Target::Desktop;
    match language {
        Language::Rust => {
            rust_items(out, t, rust);
            out.push(match crate::dap::codelldb() {
                Some(p) => Check::ok(t, "CodeLLDB (depurar na IDE)", p.display().to_string()),
                None => {
                    let check = Check::warning(t, "CodeLLDB (depurar na IDE)", "não instalado (só para depurar)");
                    match install_codelldb() {
                        Some(fix) => check.fix(fix),
                        None => check.help("Baixe o .vsix em github.com/vadimcn/codelldb/releases e extraia em <ferramentas do Aylis>/codelldb"),
                    }
                }
            });
            if cfg!(windows) {
                out.push(match crate::publish::wix() {
                    Some(p) => Check::ok(t, "WiX (instalador .msi)", p.display().to_string()),
                    None => {
                        let check = Check::warning(t, "WiX (instalador .msi)", "não instalado (só para o instalador)");
                        match (crate::dap::tools_dir(), which("dotnet")) {
                            (Some(dir), Some(_)) => check.fix(Fix::new(
                                "Instala o WiX 5 (dotnet tool) na pasta de ferramentas do Aylis",
                                &["dotnet", "tool", "install", "wix", "--version", "5.0.2", "--tool-path", &dir.join("wix").display().to_string()],
                            )),
                            _ => check.help("Precisa do .NET SDK (dotnet.microsoft.com); depois: dotnet tool install --global wix --version 5.0.2"),
                        }
                    }
                });
                out.push(match msvc_linker() {
                    Some(v) => Check::ok(t, "Linker do Visual C++ (MSVC)", v),
                    None => Check::missing(t, "Linker do Visual C++ (MSVC)", "não encontrado (o Rust no Windows usa o link.exe)").fix(crate::setup::msvc()),
                });
            } else if cfg!(target_os = "macos") {
                out.push(match output("xcode-select", &["-p"]) {
                    Some(p) => Check::ok(t, "Ferramentas de linha de comando do Xcode", p.trim()),
                    None => Check::missing(t, "Ferramentas de linha de comando do Xcode", "não encontradas")
                        .fix(Fix::new("Instala as ferramentas do Xcode", &["xcode-select", "--install"]).interactive()),
                });
            } else {
                out.push(match output("cc", &["--version"]) {
                    Some(v) => Check::ok(t, "Compilador C (linker)", v.lines().next().unwrap_or_default()),
                    None => Check::missing(t, "Compilador C (linker)", "cc não encontrado")
                        .help("Instale o gcc ou clang (ex.: sudo apt install build-essential)"),
                });
            }
        }
        Language::Python => {
            out.push(match python() {
                Some((program, (3, minor, patch))) if minor >= 8 => Check::ok(t, "Python 3.8+", format!("3.{minor}.{patch} ({program})")),
                Some((_, (major, minor, patch))) => Check::missing(t, "Python 3.8+", format!("{major}.{minor}.{patch} é antigo")).fix(install_python()),
                None => Check::missing(t, "Python 3.8+", "não encontrado").fix(install_python()),
            });
            if let Some((program, _)) = python() {
                let has = output(&program, &["-c", "import debugpy"]).is_some();
                out.push(if has {
                    Check::ok(t, "debugpy (depurar na IDE e no VS Code)", "instalado")
                } else {
                    Check::warning(t, "debugpy (depurar na IDE e no VS Code)", "não instalado (só para depurar)")
                        .fix(Fix::new("Instala o debugpy", &[program.as_str(), "-m", "pip", "install", "debugpy"]))
                });
            }
            out.push(match aylis_host.filter(|h| h.is_file()) {
                Some(h) => Check::ok(t, "aylis-host", h.display().to_string()),
                None => Check::missing(t, "aylis-host", "não encontrado ao lado da IDE").fix(Fix::new(
                    "Compila o aylis-host (na pasta do Aylis)",
                    &["cargo", "build", "-p", "aylis-host", "--manifest-path", &crate::aylis_root().join("Cargo.toml").display().to_string()],
                )),
            });
        }
        other => desktop_host(out, other, aylis_host),
    }
    // O servidor de linguagem (sugestões, declaração, erros enquanto digita no editor).
    out.push(language_server(language));
}

/// TypeScript, C#, Java, Go, C++ e Free Pascal: o compilador e o aylis-host.
fn desktop_host(out: &mut Vec<Check>, language: Language, aylis_host: Option<&Path>) {
    let t = Target::Desktop;
    let label = format!("{} (compilar o backend)", language.name());
    let toolchain = crate::hostlang::find(language);
    out.push(match &toolchain {
        Ok(tc) => Check::ok(t, &label, format!("{} ({})", tc.describe, tc.program.display())),
        Err(e) => match crate::setup::install_toolchain(language) {
            Some(fix) => Check::missing(t, &label, e.clone()).fix(fix),
            None => Check::missing(t, &label, e.clone()),
        },
    });
    out.push(host_debugger(language, toolchain.ok().map(|tc| tc.program)));
    out.push(match aylis_host.filter(|h| h.is_file()) {
        Some(h) => Check::ok(t, "aylis-host", h.display().to_string()),
        None => Check::missing(t, "aylis-host", "não encontrado ao lado da IDE").fix(Fix::new(
            "Compila o aylis-host (na pasta do Aylis)",
            &["cargo", "build", "-p", "aylis-host", "--manifest-path", &crate::aylis_root().join("Cargo.toml").display().to_string()],
        )),
    });
}

/// O depurador da linguagem (depurar na IDE): CodeLLDB (Free Pascal e C++), Delve (Go),
/// netcoredbg (C#), js-debug com o Node (TypeScript) e o `jdb` do JDK (Java).
fn host_debugger(language: Language, program: Option<PathBuf>) -> Check {
    let t = Target::Desktop;
    let (name, found) = match language {
        Language::Pascal | Language::Cpp => ("CodeLLDB (depurar na IDE)", crate::dap::codelldb()),
        Language::Go => ("Delve (depurar na IDE)", crate::dap::delve()),
        Language::CSharp => ("netcoredbg (depurar na IDE)", crate::dap::netcoredbg()),
        Language::TypeScript => ("js-debug (depurar na IDE)", crate::dap::js_debug()),
        _ => ("jdb (depurar na IDE)", crate::dap::jdb()),
    };
    if let Some(p) = found {
        if language == Language::TypeScript {
            // O js-debug só fala com o inspetor do Node (o do Bun é outro).
            if let Err(e) = crate::hostlang::node() {
                return Check::warning(t, name, format!("precisa do Node 22.6+ para depurar: {e}")).help("Instale o Node 22.6+ (nodejs.org)");
            }
        }
        return Check::ok(t, name, p.display().to_string());
    }
    let check = Check::warning(t, name, "não instalado (só para depurar)");
    let fix = match language {
        Language::Pascal | Language::Cpp => install_codelldb(),
        Language::Go => install_delve(program),
        Language::CSharp => install_netcoredbg(),
        Language::TypeScript => install_js_debug(),
        _ => None,
    };
    match fix {
        Some(fix) => check.fix(fix),
        None if language == Language::Java => check.help("O jdb vem com o JDK (Java 17+): ponha a pasta bin do JDK no PATH ou defina JAVA_HOME"),
        None => check.help("Sem instalador automático nesta plataforma"),
    }
}

/// Um `go install` na versão fixada, com o executável em `bin`. O Go confere o módulo no
/// banco de somas dele (sum.golang.org).
fn go_install(description: &str, go: &Path, module: &str, package: &str, bin: &Path) -> Fix {
    let version = crate::downloads::version(package);
    let description = format!("{description} (go install {module}@{version}) em {}", bin.display());
    // O GOBIN vai pelo ambiente do processo: nada de shell no meio.
    let mut fix = Fix::new(description, &[&go.display().to_string(), "install", &format!("{module}@{version}")]);
    fix.env.push(("GOBIN".into(), bin.display().to_string()));
    fix
}

/// O Delve pelo `go install` (versão fixada), com o executável na pasta de ferramentas do aylis.
fn install_delve(go: Option<PathBuf>) -> Option<Fix> {
    let bin = crate::dap::tools_dir()?.join("delve");
    Some(go_install("Instala o Delve", &go?, "github.com/go-delve/delve/cmd/dlv", "delve", &bin))
}

/// O netcoredbg (Samsung, ~4 MB, do GitHub), na versão fixada.
fn install_netcoredbg() -> Option<Fix> {
    Fix::download("netcoredbg", "o netcoredbg")
}

/// O servidor DAP do js-debug (o depurador de JavaScript do VS Code, do GitHub).
fn install_js_debug() -> Option<Fix> {
    Fix::download("js-debug", "o js-debug")
}

/// O servidor de linguagem do editor de código (sugestões, ir para a declaração, os erros
/// enquanto digita). Sem ele o editor funciona com o índice próprio dos fontes.
fn language_server(language: Language) -> Check {
    let t = Target::Desktop;
    let label = "Servidor de linguagem (editor de código)";
    if let Some(s) = crate::lsp::server(language, Path::new(".")) {
        return Check::ok(t, label, format!("{} ({})", s.name, s.program.display()));
    }
    let check = Check::warning(t, label, "não instalado (o editor fica sem as sugestões e os erros do servidor)");
    match install_language_server(language) {
        Some(fix) => check.fix(fix),
        None => check.help(match language {
            Language::Pascal => "Precisa do Lazarus (lazarus-ide.org) para compilar o pasls (github.com/castle-engine/pascal-language-server)",
            _ => "Sem instalador automático nesta plataforma",
        }),
    }
}

/// Instala o servidor de linguagem na pasta de ferramentas do aylis.
fn install_language_server(language: Language) -> Option<Fix> {
    let tools = crate::dap::tools_dir()?;
    let dir = |name: &str| tools.join(name).display().to_string();
    Some(match language {
        Language::Rust => Fix::new("Instala o rust-analyzer (rustup component add rust-analyzer)", &["rustup", "component", "add", "rust-analyzer"]),
        Language::Go => {
            let go = crate::hostlang::find(Language::Go).ok()?.program;
            go_install("Instala o gopls", &go, "golang.org/x/tools/gopls", "gopls", &tools.join("gopls"))
        }
        Language::Python => {
            let (python, _) = python()?;
            let version = crate::downloads::version("python-lsp-server");
            Fix::new(
                format!("Instala o python-lsp-server {version} (pip, com pyflakes, pycodestyle, autopep8 e rope) em {}", dir("pylsp")),
                &[python.as_str(), "-m", "pip", "install", "--upgrade", "--target", &dir("pylsp"), &format!("python-lsp-server[pyflakes,pycodestyle,autopep8,rope]=={version}")],
            )
        }
        Language::TypeScript => {
            // O TypeScript fica no 5 (o 7 não tem o tsserver). O npm confere a integridade.
            let (ts, tsls) = (crate::downloads::version("typescript"), crate::downloads::version("typescript-language-server"));
            let npm = if cfg!(windows) { "npm.cmd" } else { "npm" };
            Fix::new(
                format!("Instala o typescript-language-server {tsls} e o TypeScript {ts} (npm) em {}", dir("tsls")),
                &[npm, "install", "--prefix", &dir("tsls"), &format!("typescript@{ts}"), &format!("typescript-language-server@{tsls}")],
            )
        }
        Language::CSharp => {
            // Cada csharp-ls pede um .NET: o mais novo, o .NET mais novo; com o SDK 9 e o 8, os
            // últimos que rodam neles.
            let dotnet = crate::hostlang::find(Language::CSharp).ok()?.program;
            let major = crate::env::output(&dotnet.display().to_string(), &["--version"]).and_then(|v| v.trim().split('.').next()?.parse::<u32>().ok()).unwrap_or(8);
            let version = crate::downloads::version(match major {
                ..=8 => "csharp-ls-net8",
                9 => "csharp-ls-net9",
                _ => "csharp-ls",
            });
            Fix::new(
                format!("Instala o csharp-ls {version} (dotnet tool) em {}", dir("csharp-ls")),
                &[&dotnet.display().to_string(), "tool", "install", "csharp-ls", "--version", &version, "--tool-path", &dir("csharp-ls")],
            )
        }
        // O 1.38 é o último que roda no Java 17 (os de depois pedem o 21).
        Language::Java => Fix::download("jdtls", "o Eclipse JDT LS")?,
        Language::Cpp => Fix::download("clangd", "o clangd")?,
        Language::Pascal => {
            if !cfg!(windows) {
                return None;
            }
            // O pasls da Castle Engine, compilado com o Lazarus (que o winget instala), do
            // GitHub pelo git (instalado antes, se faltar).
            let src = dir("pasls-src");
            let lazbuild = crate::lsp::lazarus_dir().map(|d| d.join("lazbuild.exe")).filter(|p| p.is_file());
            let quiet = "--silent --accept-package-agreements --accept-source-agreements --disable-interactivity -e";
            let mut install = String::new();
            if lazbuild.is_none() {
                install.push_str(&format!("winget install --id Lazarus.Lazarus {quiet} & "));
            }
            if which("git").is_none() {
                install.push_str(&format!("winget install --id Git.Git {quiet} & "));
            }
            let git = crate::setup::git();
            let commit = crate::downloads::version("pascal-language-server");
            let lazbuild = lazbuild.map_or("C:\\lazarus\\lazbuild.exe".to_string(), |p| p.display().to_string());
            Fix::new(
                format!("{}Compila o pasls (Castle Engine, commit {}) com o Lazarus em {}", if install.is_empty() { "" } else { "Instala o Lazarus (winget) e " }, &commit[..commit.len().min(12)], dir("pasls")),
                &[
                    "cmd",
                    "/C",
                    &format!(
                        "{install}rmdir /s /q \"{src}\" 2>nul & \"{git}\" clone https://github.com/castle-engine/pascal-language-server \"{src}\" && \"{git}\" -C \"{src}\" checkout {commit} && \"{git}\" -C \"{src}\" submodule update --init --recursive && \"{lazbuild}\" --build-mode=Release \"{src}\\server\\pasls.lpi\" && mkdir \"{out}\" 2>nul & copy /y \"{src}\\server\\lib\\x86_64-win64\\pasls.exe\" \"{out}\\pasls.exe\"",
                        out = dir("pasls")
                    ),
                ],
            )
        }
    })
}

fn install_python() -> Fix {
    crate::setup::install_toolchain(Language::Python).unwrap_or_else(|| Fix::new("Instala o Python 3", &["python3", "--version"]))
}

/// O Python disponível: (programa, versão). No Windows também tenta o `py`.
pub fn python() -> Option<(String, (u32, u32, u32))> {
    let candidates: &[&str] = if cfg!(windows) { &["python", "py", "python3"] } else { &["python3", "python"] };
    candidates.iter().find_map(|p| {
        let v = output(p, &["--version"])?;
        // O "python" da Microsoft Store só abre a loja: não tem versão.
        Some((p.to_string(), parse_version(&v)?))
    })
}

/// O link.exe do Visual C++ (pelo vswhere, como o Rust faz).
fn msvc_linker() -> Option<String> {
    let program_files = std::env::var("ProgramFiles(x86)").ok()?;
    let vswhere = Path::new(&program_files).join("Microsoft Visual Studio/Installer/vswhere.exe");
    if vswhere.is_file() {
        let found = output(
            &vswhere.display().to_string(),
            &["-latest", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "displayName"],
        );
        if let Some(name) = found.map(|n| n.trim().to_string()).filter(|n| !n.is_empty()) {
            return Some(name);
        }
    }
    which("link").filter(|p| !p.display().to_string().contains("Git")).map(|p| p.display().to_string())
}

// ---------------------------------------------------------------------------
// Android

/// Arquiteturas de Android que o aylis compila: celulares (arm64) e o emulador (x86_64).
pub const ANDROID_ABIS: [(&str, &str); 2] = [("arm64-v8a", "aarch64-linux-android"), ("x86_64", "x86_64-linux-android")];

/// A pasta do Android SDK (`ANDROID_HOME`, `ANDROID_SDK_ROOT` ou o lugar padrão).
pub fn android_sdk() -> Option<PathBuf> {
    let from_env = ["ANDROID_HOME", "ANDROID_SDK_ROOT"].iter().find_map(std::env::var_os).map(PathBuf::from);
    let default = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(|d| PathBuf::from(d).join("Android").join("Sdk"))
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Android/sdk"))
    } else {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Android/Sdk"))
    };
    from_env.into_iter().chain(default).find(|p| p.is_dir())
}

/// A subpasta mais nova (por nome) de `dir` que satisfaz `ok`.
fn newest(dir: &Path, ok: impl Fn(&Path) -> bool) -> Option<PathBuf> {
    let mut all: Vec<PathBuf> = std::fs::read_dir(dir).ok()?.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.is_dir() && ok(p)).collect();
    all.sort_by_key(|p| version_key(&p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()));
    all.pop()
}

/// "34.0.0" e "android-34" ordenados como números.
fn version_key(name: &str) -> Vec<u64> {
    name.split(|c: char| !c.is_ascii_digit()).filter_map(|n| n.parse().ok()).collect()
}

pub fn android_ndk(sdk: &Path) -> Option<PathBuf> {
    if let Some(ndk) = std::env::var_os("ANDROID_NDK_HOME").map(PathBuf::from).filter(|p| p.is_dir()) {
        return Some(ndk);
    }
    newest(&sdk.join("ndk"), |p| p.join("source.properties").is_file())
}

pub fn android_build_tools(sdk: &Path) -> Option<PathBuf> {
    newest(&sdk.join("build-tools"), |p| p.join(exe("aapt2")).is_file())
}

pub fn android_platform(sdk: &Path) -> Option<PathBuf> {
    newest(&sdk.join("platforms"), |p| p.join("android.jar").is_file())
}

pub fn adb(sdk: &Path) -> PathBuf {
    sdk.join("platform-tools").join(exe("adb"))
}

fn sdkmanager(sdk: &Path) -> Option<PathBuf> {
    let name = if cfg!(windows) { "sdkmanager.bat" } else { "sdkmanager" };
    let latest = sdk.join("cmdline-tools/latest/bin").join(name);
    if latest.is_file() {
        return Some(latest);
    }
    newest(&sdk.join("cmdline-tools"), |p| p.join("bin").join(name).is_file()).map(|p| p.join("bin").join(name))
}

/// `nome.exe` no Windows.
pub fn exe(name: &str) -> String {
    if cfg!(windows) { format!("{name}.exe") } else { name.to_string() }
}

/// A pasta `bin` do Java (JDK): `JAVA_HOME` ou o `java` do PATH.
pub fn java_bin() -> Option<PathBuf> {
    if let Some(home) = std::env::var_os("JAVA_HOME").map(PathBuf::from) {
        let bin = home.join("bin");
        if bin.join(exe("java")).is_file() {
            return Some(bin);
        }
    }
    which("java").and_then(|j| j.parent().map(Path::to_path_buf))
}

/// Os aparelhos conectados (`adb devices`): números de série.
pub fn android_devices(sdk: &Path) -> Vec<String> {
    android_device_list(sdk).into_iter().filter(|d| d.is_ready()).map(|d| d.serial).collect()
}

/// Um aparelho que o adb vê.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AndroidDevice {
    pub serial: String,
    /// O modelo (`ASUS_X00QD`); vazio enquanto o aparelho não autoriza.
    pub model: String,
    /// `device` (pronto), `unauthorized`, `offline`...
    pub state: String,
}

impl AndroidDevice {
    pub fn is_ready(&self) -> bool {
        self.state == "device"
    }

    /// O nome para a lista da IDE: "ASUS_X00QD (JBAXB601U871Z49)", com o estado se não está
    /// pronto.
    pub fn label(&self) -> String {
        let name = if self.model.is_empty() { self.serial.clone() } else { format!("{} ({})", self.model, self.serial) };
        match self.state.as_str() {
            "device" => name,
            "unauthorized" => format!("{name} — sem autorização"),
            "offline" => format!("{name} — offline"),
            other => format!("{name} — {other}"),
        }
    }
}

/// Os aparelhos do `adb devices -l`, também os que não estão prontos (sem autorização,
/// offline), para a IDE mostrar.
pub fn android_device_list(sdk: &Path) -> Vec<AndroidDevice> {
    output(&adb(sdk).display().to_string(), &["devices", "-l"]).map(|o| parse_devices(&o)).unwrap_or_default()
}

/// Lê a saída do `adb devices -l`.
fn parse_devices(text: &str) -> Vec<AndroidDevice> {
    text.lines()
        .filter(|l| !l.starts_with("List of devices") && !l.starts_with('*'))
        .filter_map(|l| {
            let mut words = l.split_whitespace();
            let serial = words.next()?.to_string();
            let state = words.next()?.to_string();
            let model = words.find_map(|w| w.strip_prefix("model:")).unwrap_or_default().to_string();
            Some(AndroidDevice { serial, model, state })
        })
        .collect()
}

/// O estado de um aparelho (`adb get-state`): `device` quando pronto; `None` se sumiu.
pub fn android_device_state(sdk: &Path, serial: &str) -> Option<String> {
    output(&adb(sdk).display().to_string(), &["-s", serial, "get-state"]).map(|o| o.trim().to_string()).filter(|s| !s.is_empty())
}

/// Os emuladores criados (AVDs).
pub fn android_avds(sdk: &Path) -> Vec<String> {
    let emulator = sdk.join("emulator").join(exe("emulator"));
    output(&emulator.display().to_string(), &["-list-avds"])
        .map(|o| o.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with("INFO")).map(str::to_string).collect())
        .unwrap_or_default()
}

fn android(out: &mut Vec<Check>, language: Language, rust: &Rust) {
    let t = Target::Android;
    if language.uses_host() {
        // O app leva os forms; o código roda num servidor (o app casca é em Rust).
        out.push(
            Check::ok(t, "Linguagem", format!("{}: o código roda num servidor (Publicar → Servidor)", language.name()))
                .help("No servidor: aylis-host --bridge 0.0.0.0:9443 --project <projeto> -- <comando do backend>"),
        );
    }
    rust_items(out, t, rust);
    rust_targets(out, t, rust, "Alvos do Rust para Android", &ANDROID_ABIS.map(|(_, triple)| triple));
    out.push(match which("cargo-ndk") {
        Some(_) => Check::ok(t, "cargo-ndk", output("cargo", &["ndk", "--version"]).unwrap_or_default().trim().to_string()),
        None => Check::missing(t, "cargo-ndk", "não instalado").fix(Fix::new("Instala o cargo-ndk", &["cargo", "install", "cargo-ndk"])),
    });
    match java_bin().and_then(|bin| output(&bin.join(exe("java")).display().to_string(), &["-version"])) {
        Some(v) => {
            let version = v.lines().next().unwrap_or_default().to_string();
            let major = version.split('"').nth(1).and_then(|v| v.split('.').next()).and_then(|m| m.parse::<u32>().ok()).unwrap_or(0);
            out.push(if major >= 17 {
                Check::ok(t, "Java (JDK 17+)", version)
            } else {
                Check::missing(t, "Java (JDK 17+)", format!("{version} é antigo")).fix(install_jdk())
            });
        }
        None => out.push(Check::missing(t, "Java (JDK 17+)", "não encontrado").fix(install_jdk())),
    }
    out.push(match crate::publish::bundletool() {
        Some(p) => Check::ok(t, "bundletool (AAB para a Play Store)", p.display().to_string()),
        None => match crate::dap::tools_dir() {
            Some(_) => match Fix::download("bundletool", "o bundletool") {
                Some(fix) => Check::warning(t, "bundletool (AAB para a Play Store)", "não encontrado (só para o AAB)").fix(fix),
                None => Check::warning(t, "bundletool (AAB para a Play Store)", "não encontrado (só para o AAB)"),
            },
            None => Check::warning(t, "bundletool (AAB para a Play Store)", "não encontrado (só para o AAB)"),
        },
    });
    let Some(sdk) = android_sdk() else {
        out.push(
            Check::missing(t, "Android SDK", "não encontrado (defina ANDROID_HOME)").fix(
                Fix::new("Instala o Android Studio (traz o SDK; abra-o uma vez para baixar os componentes)", &["winget", "install", "--id", "Google.AndroidStudio", "-e"])
                    .interactive(),
            ),
        );
        return;
    };
    out.push(Check::ok(t, "Android SDK", sdk.display().to_string()));
    let manager = sdkmanager(&sdk);
    let sdk_fix = |package: &str, what: &str| -> Option<Fix> {
        manager.as_ref().map(|m| Fix::new(format!("Instala {what} pelo sdkmanager"), &[&m.display().to_string(), "--install", package]))
    };
    let mut item = |name: &str, found: Option<String>, package: &str, what: &str| {
        let check = match found {
            Some(v) => Check::ok(t, name, v),
            None => {
                let c = Check::missing(t, name, "não instalado");
                match sdk_fix(package, what) {
                    Some(f) => c.fix(f),
                    None => c.help(format!("Instale \"{package}\" pelo SDK Manager do Android Studio")),
                }
            }
        };
        out.push(check);
    };
    let name_of = |p: PathBuf| p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    item("Plataforma Android", android_platform(&sdk).map(name_of), "platforms;android-34", "a plataforma Android 34");
    item("Build tools", android_build_tools(&sdk).map(name_of), "build-tools;34.0.0", "as build tools 34");
    item("Platform tools (adb)", adb(&sdk).is_file().then(|| "adb".to_string()), "platform-tools", "as platform tools");
    item("NDK", android_ndk(&sdk).map(name_of), "ndk;27.2.12479018", "o NDK 27");
    if adb(&sdk).is_file() {
        let devices = android_devices(&sdk);
        if devices.is_empty() {
            let avds = android_avds(&sdk);
            let check = Check::warning(t, "Aparelho conectado", "nenhum (só para Executar; publicar o APK não precisa)");
            out.push(match avds.first() {
                Some(avd) => check.fix(
                    Fix::new(format!("Abre o emulador {avd}"), &[&sdk.join("emulator").join(exe("emulator")).display().to_string(), "-avd", avd])
                        .interactive(),
                ),
                None => check.help("Conecte um celular com a depuração USB ligada, ou crie um emulador no Android Studio (Device Manager)"),
            });
        } else {
            out.push(Check::ok(t, "Aparelho conectado", devices.join(", ")));
        }
    }
}

fn install_jdk() -> Fix {
    if cfg!(windows) {
        Fix::new("Instala o OpenJDK 17 (winget)", &["winget", "install", "--id", "Microsoft.OpenJDK.17", "-e"]).interactive()
    } else if cfg!(target_os = "macos") {
        Fix::new("Instala o OpenJDK 17 (Homebrew)", &["brew", "install", "openjdk@17"])
    } else {
        Fix::new("Instala o OpenJDK 17 (apt)", &["sudo", "apt", "install", "-y", "openjdk-17-jdk"]).interactive()
    }
}

// ---------------------------------------------------------------------------
// Web

fn web(out: &mut Vec<Check>, language: Language, rust: &Rust) {
    let t = Target::Web;
    if language.uses_host() {
        out.push(
            Check::ok(t, "Linguagem", format!("{}: o código roda num servidor (Publicar → Servidor)", language.name()))
                .help("No servidor: aylis-host --bridge 0.0.0.0:9443 --project <projeto> -- <comando do backend>"),
        );
    }
    rust_items(out, t, rust);
    rust_targets(out, t, rust, "Alvo do Rust para WebAssembly", &["wasm32-unknown-unknown"]);
    let wanted = wasm_bindgen_version();
    let found = output("wasm-bindgen", &["--version"]).and_then(|v| v.split_whitespace().nth(1).map(str::to_string));
    let install = |v: &str| Fix::new(format!("Instala o wasm-bindgen-cli {v}"), &["cargo", "install", "wasm-bindgen-cli", "--version", v, "--locked"]);
    out.push(match (found, wanted) {
        (Some(f), Some(w)) if f == w => Check::ok(t, "wasm-bindgen", f),
        (Some(f), Some(w)) => Check::missing(t, "wasm-bindgen", format!("{f} instalado; o Aylis usa {w} (precisam ser iguais)")).fix(install(&w)),
        (Some(f), None) => Check::ok(t, "wasm-bindgen", f),
        (None, Some(w)) => Check::missing(t, "wasm-bindgen", "não instalado").fix(install(&w)),
        (None, None) => Check::missing(t, "wasm-bindgen", "não instalado").fix(Fix::new("Instala o wasm-bindgen-cli", &["cargo", "install", "wasm-bindgen-cli"])),
    });
}

/// A versão do `wasm-bindgen` que o aylis usa (do `Cargo.lock` dele).
pub fn wasm_bindgen_version() -> Option<String> {
    lock_version(&crate::aylis_root().join("Cargo.lock"), "wasm-bindgen")
}

/// Versão de um pacote num `Cargo.lock`.
pub fn lock_version(lock: &Path, package: &str) -> Option<String> {
    let text = std::fs::read_to_string(lock).ok()?;
    let marker = format!("name = \"{package}\"\nversion = \"");
    let start = text.find(&marker)? + marker.len();
    Some(text[start..].split('"').next()?.to_string())
}

// ---------------------------------------------------------------------------
// iOS

fn ios(out: &mut Vec<Check>, language: Language, rust: &Rust) {
    let t = Target::Ios;
    if !cfg!(target_os = "macos") {
        out.push(Check::missing(t, "macOS", "o app de iOS só é gerado num Mac (exigência da Apple: Xcode)").help("Abra o projeto na IDE do Aylis num Mac"));
        return;
    }
    if language.uses_host() {
        out.push(Check::missing(t, "Linguagem", format!("projetos em {} ainda não publicam para o iOS", language.name())).help("Use Rust, ou publique para o navegador (o iPhone abre)."));
        return;
    }
    rust_items(out, t, rust);
    rust_targets(out, t, rust, "Alvos do Rust para iOS", &["aarch64-apple-ios", "aarch64-apple-ios-sim"]);
    out.push(match output("xcrun", &["--sdk", "iphoneos", "--show-sdk-version"]) {
        Some(v) => Check::ok(t, "Xcode (SDK do iOS)", v.trim()),
        None => Check::missing(t, "Xcode (SDK do iOS)", "não encontrado").help("Instale o Xcode pela App Store e abra-o uma vez"),
    });
    let booted = output("xcrun", &["simctl", "list", "devices", "booted"]).is_some_and(|o| o.contains("Booted"));
    out.push(if booted {
        Check::ok(t, "Simulador aberto", "sim")
    } else {
        Check::warning(t, "Simulador aberto", "nenhum (só para Executar)").fix(Fix::new("Abre o Simulador", &["open", "-a", "Simulator"]).interactive())
    });
}

// ---------------------------------------------------------------------------
// Linux pelo WSL

fn linux_wsl(out: &mut Vec<Check>, language: Language) {
    let t = Target::Linux;
    if !cfg!(windows) {
        out.push(Check::ok(t, "WSL", "não se aplica (fora do Windows, a saída do sistema já é o Linux ou o macOS)"));
        return;
    }
    if language != Language::Rust {
        out.push(Check::missing(t, "Linguagem", format!("projetos em {} ainda não rodam no Linux pelo WSL", language.name())).help("Use Rust para gerar o executável Linux."));
        return;
    }
    if !crate::wsl::available() {
        out.push(Check::missing(t, "WSL", "não instalado").fix(Fix::new("Instala o WSL com o Ubuntu (pede permissão de administrador e reiniciar)", &["wsl.exe", "--install", "-d", "Ubuntu"]).interactive()));
        return;
    }
    let Some(distro) = crate::wsl::distro() else {
        out.push(Check::missing(t, "Distribuição Linux", "nenhuma (a docker-desktop não serve)").fix(Fix::new("Instala o Ubuntu no WSL", &["wsl.exe", "--install", "-d", "Ubuntu", "--no-launch"])));
        return;
    };
    out.push(Check::ok(t, "Distribuição Linux", &distro));
    let run = |cmd: String| ["wsl.exe".to_string(), "-d".into(), distro.clone(), "--exec".into(), "bash".into(), "-lc".into(), cmd];
    let fix = |description: &str, cmd: String| Fix { description: description.into(), command: run(cmd).to_vec(), interactive: false, fetch: None, env: Vec::new() };
    // As bibliotecas (o que falta pelo dpkg).
    let list = crate::wsl::PACKAGES.join(" ");
    let missing = crate::wsl::shell_output(&distro, &format!("for p in {list}; do dpkg -s $p >/dev/null 2>&1 || echo $p; done")).unwrap_or_default();
    let missing: Vec<&str> = missing.split_whitespace().collect();
    out.push(if missing.is_empty() {
        Check::ok(t, "Bibliotecas de compilação", format!("{} pacotes", crate::wsl::PACKAGES.len()))
    } else {
        Check::missing(t, "Bibliotecas de compilação", format!("faltam {}", missing.join(", ")))
            .fix(fix("Instala as bibliotecas pelo apt (como root)", format!("export DEBIAN_FRONTEND=noninteractive; (apt-get update && apt-get install -y {list}) || sudo apt-get install -y {list}")))
    });
    let cargo = crate::wsl::shell_output(&distro, "[ -f \"$HOME/.cargo/env\" ] && . \"$HOME/.cargo/env\"; cargo --version");
    out.push(match cargo {
        Some(v) => Check::ok(t, "Rust no Linux", v.trim()),
        None => Check::missing(t, "Rust no Linux", "não instalado")
            .fix(fix("Instala o Rust no Linux (rustup)", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal".into())),
    });
    let wslg = crate::wsl::shell_output(&distro, "ls /mnt/wslg/runtime-dir/wayland-0 >/dev/null 2>&1 && echo sim").is_some_and(|o| o.contains("sim"));
    out.push(if wslg {
        Check::ok(t, "Janela (WSLg)", "sim")
    } else {
        Check::warning(t, "Janela (WSLg)", "não encontrado: o app compila, mas a janela pode não aparecer").help("Atualize o WSL: wsl --update")
    });
}

// ---------------------------------------------------------------------------

/// Como [`output`], dizendo por que falhou: `Err(None)` se o programa não existe,
/// `Err(Some(mensagem))` se rodou e falhou (o stderr).
pub fn run(program: &str, args: &[&str]) -> Result<String, Option<String>> {
    let mut command = Command::new(program);
    command.args(args);
    let out = quiet(&mut command).output().map_err(|_| None)?;
    if !out.status.success() {
        let text = if out.stderr.is_empty() { out.stdout } else { out.stderr };
        return Err(Some(String::from_utf8_lossy(&text).trim().to_string()).filter(|t| !t.is_empty()));
    }
    let text = if out.stdout.is_empty() { out.stderr } else { out.stdout };
    Ok(String::from_utf8_lossy(&text).to_string())
}

/// Saída de um comando que terminou bem (stdout, ou stderr se o stdout veio vazio, como o
/// `java -version`). `None` se não existe ou falhou.
pub fn output(program: &str, args: &[&str]) -> Option<String> {
    let mut command = Command::new(program);
    command.args(args);
    let out = quiet(&mut command).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = if out.stdout.is_empty() { out.stderr } else { out.stdout };
    Some(String::from_utf8_lossy(&text).to_string())
}

/// A primeira versão `a.b.c` de um texto.
pub fn parse_version(text: &str) -> Option<(u32, u32, u32)> {
    text.split(|c: char| !(c.is_ascii_digit() || c == '.')).find_map(|word| {
        let mut parts = word.split('.').map(|p| p.parse::<u32>());
        match (parts.next(), parts.next(), parts.next()) {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c))) => Some((a, b, c)),
            (Some(Ok(a)), Some(Ok(b)), None) => Some((a, b, 0)),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_adb_device_list_keeps_model_and_state() {
        let text = "List of devices attached\nJBAXB601U871Z49        device product:WW_X00QD model:ASUS_X00QD device:ASUS_X00QD transport_id:2\nemulator-5554 offline transport_id:3\nR58M123 unauthorized usb:1-1 transport_id:4\n\n";
        let list = parse_devices(text);
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].label(), "ASUS_X00QD (JBAXB601U871Z49)");
        assert!(list[0].is_ready() && !list[1].is_ready());
        assert_eq!(list[1].label(), "emulator-5554 — offline");
        assert_eq!(list[2].label(), "R58M123 — sem autorização");
        assert!(parse_devices("* daemon started successfully\nList of devices attached\n").is_empty());
    }
}
