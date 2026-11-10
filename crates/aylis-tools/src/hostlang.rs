//! Compilar e executar os projetos em TypeScript, C#, Java, Go, C++ e Free Pascal: acha o
//! compilador de cada uma, compila o backend e devolve o comando que o `aylis-host` inicia.
//!
//! Fora do PATH: o caminho escolhido na aba da linguagem nas Opções da IDE
//! ([`crate::settings`]); depois o PATH e as pastas padrão de cada instalação.

use std::path::{Path, PathBuf};
use std::process::Command;

use aylis_project::{Language, Project};

use crate::env::exe;
use crate::process::{Control, Event, LineKind, Sink, run_tool};

/// O compilador (ou o executor) achado para a linguagem.
#[derive(Clone, Debug)]
pub struct Toolchain {
    pub language: Language,
    /// O programa principal (bun/node, dotnet, javac, go, vcvars64.bat ou g++, fpc).
    pub program: PathBuf,
    /// O auxiliar: `java` (Java), as units (Free Pascal).
    pub extra: Option<PathBuf>,
    /// Como aparece no Verificar ambiente.
    pub describe: String,
}

/// Procura `name` no PATH (com `.exe` no Windows).
pub fn which(name: &str) -> Option<PathBuf> {
    let file = exe(name);
    std::env::var_os("PATH").and_then(|paths| std::env::split_paths(&paths).map(|d| d.join(&file)).find(|p| p.is_file()))
}

fn version(program: &Path, args: &[&str]) -> Option<String> {
    let out = crate::process::quiet(Command::new(program).args(args)).output().ok()?;
    let text = if out.stdout.is_empty() { out.stderr } else { out.stdout };
    Some(String::from_utf8_lossy(&text).lines().next().unwrap_or_default().trim().to_string()).filter(|s| !s.is_empty())
}

/// O `vcvars64.bat` do Visual C++ (pelo vswhere).
fn vcvars() -> Option<PathBuf> {
    if let Some(p) = crate::settings::tool("vcvars") {
        return Some(p);
    }
    let vswhere = PathBuf::from(std::env::var_os("ProgramFiles(x86)")?).join("Microsoft Visual Studio/Installer/vswhere.exe");
    let out = crate::process::quiet(&mut Command::new(vswhere))
        .args(["-latest", "-products", "*", "-requires", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"])
        .output()
        .ok()?;
    let dir = String::from_utf8_lossy(&out.stdout).trim().to_string();
    Some(PathBuf::from(dir).join("VC/Auxiliary/Build/vcvars64.bat")).filter(|p| p.is_file())
}

/// O Node 22.6+ (roda TypeScript direto): o escolhido nas Opções da IDE ou o do PATH. O depurador da IDE
/// usa o Node mesmo com o Bun instalado (o js-debug só fala com o inspetor do Node).
pub fn node() -> Result<(PathBuf, String), String> {
    let node = crate::settings::tool("node").or_else(|| which("node")).ok_or("nem Bun nem Node encontrados (instale o Bun: bun.sh, ou o Node 22.6+)")?;
    let v = version(&node, &["--version"]).unwrap_or_default();
    let mut parts = v.trim_start_matches('v').split('.').map(|p| p.parse::<u32>().unwrap_or(0));
    let (major, minor) = (parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    if major < 22 || (major == 22 && minor < 6) {
        return Err(format!("Node {v} não roda TypeScript direto: precisa do 22.6+ (ou do Bun)"));
    }
    Ok((node, v))
}

/// Acha as ferramentas da linguagem, ou diz o que falta.
pub fn find(language: Language) -> Result<Toolchain, String> {
    let tc = |program: PathBuf, extra: Option<PathBuf>, describe: String| Ok(Toolchain { language, program, extra, describe });
    match language {
        Language::TypeScript => {
            if let Some(bun) = crate::settings::tool("bun").or_else(|| which("bun")).or_else(|| home(".bun/bin").map(|d| d.join(exe("bun"))).filter(|p| p.is_file())) {
                let v = version(&bun, &["--version"]).unwrap_or_default();
                return tc(bun, None, format!("Bun {v}"));
            }
            let (node, v) = node()?;
            tc(node, None, format!("Node {v}"))
        }
        Language::CSharp => {
            let dotnet = crate::settings::tool("dotnet")
                .or_else(|| which("dotnet"))
                .or_else(|| home(".dotnet").map(|d| d.join(exe("dotnet"))).filter(|p| p.is_file()))
                .or_else(|| std::env::var_os("ProgramFiles").map(|p| PathBuf::from(p).join("dotnet").join(exe("dotnet"))).filter(|p| p.is_file()))
                .ok_or("SDK do .NET não encontrado (dotnet.microsoft.com, .NET 8)")?;
            let v = version(&dotnet, &["--version"]).unwrap_or_default();
            tc(dotnet, None, format!(".NET SDK {v}"))
        }
        Language::Java => {
            let java_home = std::env::var_os("JAVA_HOME").map(|h| PathBuf::from(h).join("bin"));
            let javac = crate::settings::tool("javac").or_else(|| which("javac")).or_else(|| java_home.as_ref().map(|b| b.join(exe("javac"))).filter(|p| p.is_file())).ok_or("JDK não encontrado (javac; o Java 17+)")?;
            let java = crate::settings::tool("java").or_else(|| javac.parent().map(|d| d.join(exe("java")))).filter(|p| p.is_file()).ok_or("java não encontrado ao lado do javac")?;
            let v = version(&javac, &["-version"]).unwrap_or_default();
            tc(javac, Some(java), v)
        }
        Language::Go => {
            let go = crate::settings::tool("go")
                .or_else(|| which("go"))
                .or_else(|| std::env::var_os("ProgramFiles").map(|p| PathBuf::from(p).join("Go/bin").join(exe("go"))).filter(|p| p.is_file()))
                .ok_or("Go não encontrado (go.dev/dl)")?;
            let v = version(&go, &["version"]).unwrap_or_default();
            tc(go, None, v)
        }
        Language::Cpp => {
            if cfg!(windows) {
                if let Some(bat) = vcvars() {
                    return tc(bat, None, "Visual C++ (cl)".into());
                }
            }
            let gxx = crate::settings::tool("gxx").or_else(|| which("g++")).or_else(|| which("clang++")).ok_or("compilador C++ não encontrado (Visual C++ Build Tools, g++ ou clang++)")?;
            let v = version(&gxx, &["--version"]).unwrap_or_default();
            tc(gxx, None, v)
        }
        Language::Pascal => {
            let fpc = crate::settings::tool("fpc").or_else(|| which("fpc")).or_else(fpc_default).ok_or("Free Pascal não encontrado (freepascal.org, ou o do Lazarus)")?;
            let v = version(&fpc, &["-iV"]).map(|v| format!("Free Pascal {v}")).unwrap_or_default();
            // Sem fpc.cfg (instalação sem configurar), as units vão na linha de comando.
            let units = crate::settings::tool("fpc_units").or_else(|| {
                let cfg = fpc.parent()?.join("fpc.cfg");
                if cfg.is_file() {
                    return None;
                }
                let root = fpc.parent()?.parent()?.parent()?;
                Some(root.join("units").join(if cfg!(windows) { "x86_64-win64" } else { "x86_64-linux" })).filter(|p| p.is_dir())
            });
            tc(fpc, units, v)
        }
        Language::Rust | Language::Python => Err("Rust e Python têm o próprio caminho".into()),
    }
}

fn home(sub: &str) -> Option<PathBuf> {
    std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(|h| PathBuf::from(h).join(sub))
}

/// `C:\FPC\<versão>\bin\i386-win32\fpc.exe` (onde o instalador põe).
fn fpc_default() -> Option<PathBuf> {
    // O do Lazarus primeiro (64 bits, já configurado; é o que o "Configurar linguagens" instala).
    if let Some(fpc) = crate::lsp::lazarus_dir().and_then(|l| std::fs::read_dir(l.join("fpc")).ok()).and_then(|d| {
        let mut versions: Vec<PathBuf> = d.flatten().map(|e| e.path()).collect();
        versions.sort();
        versions.into_iter().rev().map(|v| v.join("bin/x86_64-win64/fpc.exe")).find(|p| p.is_file())
    }) {
        return Some(fpc);
    }
    let dir = std::fs::read_dir("C:/FPC").ok()?;
    let mut versions: Vec<PathBuf> = dir.flatten().map(|e| e.path()).collect();
    versions.sort();
    versions.into_iter().rev().map(|v| v.join("bin/i386-win32/fpc.exe")).find(|p| p.is_file())
}

/// Todos os arquivos com a extensão debaixo de `dir`.
fn files(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            files(&p, ext, out);
        } else if p.extension().is_some_and(|x| x == ext) {
            out.push(p);
        }
    }
}

/// Compila o backend em `out` (TypeScript não compila) e devolve o comando que o
/// aylis-host inicia, com caminhos relativos à pasta do projeto quando `relative`.
pub fn build(project: &Project, tc: &Toolchain, out: &Path, release: bool, control: &Control, sink: Sink) -> Result<Vec<String>, String> {
    let dir = project.code_dir();
    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let s = |p: &Path| p.display().to_string();
    sink(Event::Step(format!("Compilando {} ({})…", project.name, tc.language.name())));
    match tc.language {
        Language::TypeScript => {
            let is_bun = tc.program.file_stem().is_some_and(|n| n.eq_ignore_ascii_case("bun"));
            Ok(if is_bun {
                vec![s(&tc.program), "src/main.ts".into()]
            } else {
                vec![s(&tc.program), "--experimental-strip-types".into(), "--no-warnings".into(), "src/main.ts".into()]
            })
        }
        Language::CSharp => {
            let mut c = Command::new(&tc.program);
            c.current_dir(&dir).args(["build", "App.csproj", "-v", "q", "-nologo", "-c", if release { "Release" } else { "Debug" }, "-o"]).arg(out);
            c.env("DOTNET_CLI_TELEMETRY_OPTOUT", "1").env("DOTNET_NOLOGO", "1");
            run_tool(&mut c, control, sink, LineKind::Tool)?;
            Ok(vec![s(&tc.program), s(&out.join("App.dll"))])
        }
        Language::Java => {
            let mut sources = Vec::new();
            files(&dir.join("src"), "java", &mut sources);
            // O cliente (`dev.aylis`) vem do aylis, não do projeto (item 12).
            files(&Path::new(&aylis_project::codegen::aylis_root()).join("clients/java/dev/aylis"), "java", &mut sources);
            let mut c = Command::new(&tc.program);
            c.current_dir(&dir).args(["-encoding", "UTF-8"]);
            if !release {
                // As variáveis locais para o depurador (jdb).
                c.arg("-g");
            }
            c.arg("-d").arg(out).args(&sources);
            run_tool(&mut c, control, sink, LineKind::Tool)?;
            let java = tc.extra.clone().unwrap_or_else(|| PathBuf::from("java"));
            Ok(vec![s(&java), "-cp".into(), s(out), "Main".into()])
        }
        Language::Go => {
            let program = out.join(exe("app"));
            let mut c = Command::new(&tc.program);
            c.current_dir(&dir).arg("build");
            if !release {
                // Sem otimizar: o depurador (Delve) vê as linhas e as variáveis como estão no código.
                c.arg("-gcflags=all=-N -l");
            }
            c.arg("-o").arg(&program).arg(".");
            run_tool(&mut c, control, sink, LineKind::Tool)?;
            Ok(vec![s(&program)])
        }
        Language::Cpp => {
            let program = out.join(exe("app"));
            let mut sources = vec![dir.join("src/main.cpp")];
            files(&dir.join("src/forms"), "cpp", &mut sources);
            if tc.program.extension().is_some_and(|e| e.eq_ignore_ascii_case("bat")) {
                // Visual C++: um .bat que prepara o ambiente e chama o cl.
                let objs = out.join("obj");
                std::fs::create_dir_all(&objs).map_err(|e| e.to_string())?;
                // Num .bat, `%` expande variável: um nome de pasta com `%` vira `%%`.
                let bat_path = |p: &Path| p.display().to_string().replace('%', "%%");
                let list: Vec<String> = sources.iter().map(|p| format!("\"{}\"", bat_path(p))).collect();
                let script = format!(
                    "@call \"{}\" >nul\r\ncl /nologo /std:c++17 /EHsc /utf-8 {} {} /Fe:\"{}\" /Fo:\"{}/\"\r\n",
                    bat_path(&tc.program),
                    if release { "/O2" } else { "/Od /Zi" },
                    list.join(" "),
                    bat_path(&program),
                    bat_path(&objs)
                );
                let bat = out.join("compilar.bat");
                std::fs::write(&bat, script).map_err(|e| e.to_string())?;
                run_tool(Command::new("cmd").current_dir(&dir).arg("/C").arg(&bat), control, sink, LineKind::Tool)?;
            } else {
                let mut c = Command::new(&tc.program);
                c.current_dir(&dir).arg("-std=c++17").arg(if release { "-O2" } else { "-g" }).args(&sources).arg("-o").arg(&program);
                run_tool(&mut c, control, sink, LineKind::Tool)?;
            }
            Ok(vec![s(&program)])
        }
        Language::Pascal => {
            let units = out.join("units");
            std::fs::create_dir_all(&units).map_err(|e| e.to_string())?;
            let mut c = Command::new(&tc.program);
            c.current_dir(&dir).arg("-Mobjfpc");
            if cfg!(windows) {
                c.arg("-Px86_64");
            }
            if let Some(u) = &tc.extra {
                c.arg(format!("-Fu{}", u.join("*").display())).arg(format!("-Fu{}", u.join("rtl").display()));
            }
            // Desenvolvimento: informação de depuração DWARF (o LLDB da IDE lê).
            c.arg(if release { "-O2" } else { "-gw3" }).arg("-Fusrc").arg("-Fusrc/forms").arg(format!("-FU{}", units.display())).arg(format!("-FE{}", out.display())).arg("src/app.lpr");
            run_tool(&mut c, control, sink, LineKind::Tool)?;
            Ok(vec![s(&out.join(exe("app")))])
        }
        Language::Rust | Language::Python => Err("Rust e Python têm o próprio caminho".into()),
    }
}
