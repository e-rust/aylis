//! Publicar: o aplicativo final de cada plataforma, em `<projeto>/dist/<plataforma>/`.
//!
//! | Plataforma | Rust | Python |
//! |---|---|---|
//! | Windows/Linux/macOS | um executável só (forms e assets embutidos; ícone no .exe) | pasta com o `aylis-host`, o projeto e um atalho (precisa do Python na máquina) |
//! | Android | APK assinado (arm64 + x86_64) | — |
//! | Navegador | pasta com `index.html` e o `.wasm` (sirva com qualquer servidor) | — |
//! | iOS | `.app` para o simulador (num Mac) | — |

use std::path::{Path, PathBuf};
use std::process::Command;

use aylis_core::Color;
use aylis_project::{Language, Project};

use crate::env::{self, exe};
use crate::process::{Control, Event, LineKind, Sink, cargo, run_tool};
use crate::{Target, copy_dir, file_name};

// Os instaladores `.exe` do Windows (NSIS e Inno Setup); o `.msi` fica aqui.
#[path = "win_installer.rs"]
pub mod win_installer;

/// Opções de uma publicação.
#[derive(Clone, Debug)]
pub struct Options {
    pub target: Target,
    /// Pasta de saída. `None` = `<projeto>/dist/<plataforma>`.
    pub out_dir: Option<PathBuf>,
    /// Compilação de desenvolvimento (mais rápida, sem otimizações): usada pelo Executar.
    pub debug: bool,
    /// Android: arquiteturas (`arm64-v8a`, `x86_64`). Vazio = todas.
    pub abis: Vec<String>,
    /// O `aylis-host` (projetos Python no desktop).
    pub aylis_host: Option<PathBuf>,
    /// Features a mais do projeto (o Executar liga `inspect`: o canal com a IDE).
    pub features: Vec<String>,
    /// Android: AAB (o pacote da Play Store) em vez de APK.
    pub android_bundle: bool,
    /// Windows: também o instalador, do tipo do projeto (`publish.windows.installer`: o `.msi`,
    /// o NSIS ou o Inno Setup).
    pub installer: bool,
    /// Android: a chave do desenvolvedor (com as senhas). `None`: a de depuração.
    pub signing: Option<Signing>,
    /// O crate compilado no lugar do projeto (o app casca dos projetos em outras
    /// linguagens, no celular e no navegador).
    pub crate_dir: Option<PathBuf>,
}

impl Options {
    pub fn new(target: Target) -> Self {
        Self { target, out_dir: None, debug: false, abis: Vec::new(), aylis_host: None, features: Vec::new(), android_bundle: false, installer: false, signing: None, crate_dir: None }
    }
}

/// A chave de assinatura do Android e as senhas (só em memória).
#[derive(Clone, Debug)]
pub struct Signing {
    pub keystore: PathBuf,
    pub alias: String,
    pub store_password: String,
    pub key_password: String,
}

impl Signing {
    /// A chave do projeto com as senhas das variáveis `AYLIS_KEYSTORE_PASSWORD` e
    /// `AYLIS_KEY_PASSWORD` (sem a segunda, vale a primeira). `None` se o projeto não tem
    /// chave ou falta a senha.
    pub fn from_env(project: &Project) -> Option<Signing> {
        if project.publish.keystore.is_empty() {
            return None;
        }
        let store_password = std::env::var("AYLIS_KEYSTORE_PASSWORD").ok()?;
        let key_password = std::env::var("AYLIS_KEY_PASSWORD").unwrap_or_else(|_| store_password.clone());
        Some(Signing { keystore: project.file(&project.publish.keystore), alias: project.publish.key_alias.clone(), store_password, key_password })
    }
}

/// Cria uma chave de assinatura (como o "Generate Signed Bundle" do Android Studio): RSA
/// 2048, válida por 25 anos. `name` vai no certificado (`CN`).
pub fn create_keystore(path: &Path, alias: &str, password: &str, name: &str, control: &Control, sink: Sink) -> Result<(), String> {
    let java = env::java_bin().ok_or("Java (JDK 17+) não encontrado")?;
    if path.exists() {
        return Err(format!("{} já existe", path.display()));
    }
    if password.chars().count() < 6 {
        return Err("a senha precisa ter pelo menos 6 caracteres".into());
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let dname = format!("CN={}", name.replace(',', " "));
    run_tool(
        Command::new(java.join(exe("keytool")))
            .args(["-genkeypair", "-keystore"])
            .arg(path)
            .args(["-storetype", "PKCS12", "-alias", alias, "-keyalg", "RSA", "-keysize", "2048", "-validity", "9125", "-dname", &dname])
            .args(["-storepass", password, "-keypass", password]),
        control,
        sink,
        LineKind::Tool,
    )
}

/// O bundletool do Google (monta o AAB): o escolhido nas Opções da IDE ou na pasta de
/// ferramentas do aylis.
pub fn bundletool() -> Option<PathBuf> {
    crate::settings::tool("bundletool")
        .or_else(|| crate::dap::tools_dir().map(|d| d.join("bundletool.jar")))
        .filter(|p| p.is_file())
}

/// O WiX (monta o .msi): o escolhido nas Opções da IDE, a pasta de ferramentas do aylis ou o PATH.
pub fn wix() -> Option<PathBuf> {
    crate::settings::tool("wix")
        .or_else(|| crate::dap::tools_dir().map(|d| d.join("wix").join(exe("wix"))))
        .filter(|p| p.is_file())
        .or_else(|| crate::hostlang::which("wix"))
}

/// O que foi gerado.
#[derive(Clone, Debug)]
pub struct Published {
    /// O arquivo principal (o .exe, o .apk, o index.html, o .app).
    pub main: PathBuf,
    /// A pasta com tudo.
    pub dir: PathBuf,
}

/// Publica o projeto. Demora (compila em modo otimizado): rode numa thread de trabalho.
pub fn publish(project: &Project, options: &Options, control: &Control, sink: Sink) -> Result<Published, String> {
    let out = options.out_dir.clone().unwrap_or_else(|| project.dir().join("dist").join(options.target.id()));
    // O `.ico` do Windows com todos os tamanhos (o build põe no .exe) sai antes de compilar.
    if !project.publish.icon.is_empty() {
        let icon = crate::icons::Icon::open(&project.file(&project.publish.icon))?;
        for w in icon.warnings() {
            sink(Event::Line(format!("ícone: {w}"), LineKind::Tool));
        }
        crate::icons::windows(&icon, &project.dir().join(aylis_build::APP_ICON))?;
    }
    // Outras linguagens no celular e no navegador: o app casca liga a tela ao servidor.
    let mut shell_options;
    let options = if project.language.uses_host() && matches!(options.target, Target::Android | Target::Web) {
        if project.publish.backend_url.is_empty() {
            return Err(format!(
                "projetos em {} rodam o código num servidor no celular e no navegador: informe o endereço dele (Publicar → Servidor, wss://...) \
                 e rode lá: aylis-host --bridge 0.0.0.0:9443 --project {} -- <o comando do backend>",
                project.language.name(),
                project.path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
            ));
        }
        aylis_project::codegen::generate(project)?;
        shell_options = options.clone();
        shell_options.crate_dir = Some(aylis_project::codegen::remote_shell(project, &project.publish.backend_url)?);
        &shell_options
    } else if options.target != Target::Desktop && project.language.uses_host() {
        return Err(format!("projetos em {} não publicam para {} (troque a linguagem para Rust)", project.language.name(), options.target.name()));
    } else {
        options
    };
    if project.language == Language::Rust {
        aylis_project::codegen::generate(project)?;
        if !project.code_dir().join("src/lib.rs").is_file() {
            return Err("o projeto não tem src/lib.rs (foi criado por uma versão antiga da IDE e o main.rs foi editado): \
                        mova o código para src/lib.rs com uma função app(), como num projeto novo"
                .into());
        }
    }
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).map_err(|e| format!("{}: {e}", out.display()))?;
    match (options.target, project.language) {
        (Target::Desktop, Language::Rust) => desktop_rust(project, options, &out, control, sink),
        (Target::Desktop, Language::Python) => desktop_python(project, options, &out, sink),
        (Target::Desktop, _) => desktop_host(project, options, &out, control, sink),
        (Target::Android, _) => android(project, options, &out, control, sink),
        (Target::Web, _) => web(project, options, &out, control, sink),
        (Target::Ios, _) => ios(project, options, &out, control, sink),
        (Target::Linux, Language::Rust) => linux_wsl(project, options, &out, control, sink),
        (Target::Linux, _) => Err("o Linux pelo WSL só publica projetos em Rust".into()),
    }
}

/// Linux pelo WSL: o executável otimizado e, com `installer`, o pacote `.deb`.
fn linux_wsl(project: &Project, options: &Options, out: &Path, control: &Control, sink: Sink) -> Result<Published, String> {
    let distro = crate::wsl::distro().ok_or("nenhuma distribuição Linux no WSL (Ferramentas → Verificar ambiente instala o Ubuntu)")?;
    let app_file = file_name(project.app_name());
    let script = crate::wsl::publish_script(project, out, &app_file, options.installer);
    match crate::wsl::run(&distro, &script, control, sink, false)? {
        Some(0) | None => {}
        Some(3) => return Err("o Rust não está instalado no Linux do WSL (Ferramentas → Verificar ambiente instala)".into()),
        Some(c) => return Err(format!("a compilação no WSL terminou com código {c} (veja as mensagens)")),
    }
    let exe = out.join(&app_file);
    sink(Event::Line(format!("{} ({})", exe.display(), size(&exe)), LineKind::Tool));
    let deb = std::fs::read_dir(out).ok().and_then(|d| d.flatten().map(|e| e.path()).find(|p| p.extension().is_some_and(|x| x == "deb")));
    Ok(Published { main: deb.unwrap_or(exe), dir: out.to_path_buf() })
}

fn profile(options: &Options) -> &'static str {
    if options.debug { "debug" } else { "release" }
}

// ---------------------------------------------------------------------------
// Desktop

fn desktop_rust(project: &Project, options: &Options, out: &Path, control: &Control, sink: Sink) -> Result<Published, String> {
    sink(Event::Step(format!("Compilando {} ({})…", project.name, if options.debug { "desenvolvimento" } else { "otimizado" })));
    // O binário do projeto (`output.bin`, ou o `[[bin]]`; I08 do Tabernáculo).
    let package = project.bin_name();
    let mut args = vec!["build", "--bin", package.as_str()];
    if !options.debug {
        args.push("--release");
    }
    let built = cargo("cargo", &args, &project.code_dir(), &[], control, sink)?;
    let exe_path = built
        .artifacts
        .iter()
        .filter(|a| a.kinds.iter().any(|k| k == "bin"))
        .find_map(|a| a.executable.clone())
        .ok_or("o cargo não informou o executável")?;
    let name = format!("{}{}", file_name(project.app_name()), std::env::consts::EXE_SUFFIX);
    let dest = out.join(&name);
    std::fs::copy(&exe_path, &dest).map_err(|e| format!("{}: {e}", dest.display()))?;
    sink(Event::Line(format!("{} ({})", dest.display(), size(&dest)), LineKind::Tool));
    let mut extra = ray_tracing_dlls(project, out, sink)?;
    // Os arquivos que o projeto pede ao lado do executável (DLLs de terceiros).
    if cfg!(windows) {
        extra.extend(win_installer::copy_extra_files(project, out, sink)?);
    }
    if options.installer && cfg!(windows) {
        // O tipo de instalador do projeto (`publish.windows.installer`): o `.msi` é o padrão.
        let main = match project.publish.windows.kind() {
            aylis_project::WindowsInstaller::Msi => windows_installer(project, &dest, &extra, out, control, sink)?,
            aylis_project::WindowsInstaller::Nsis => win_installer::nsis(project, &dest, &extra, out, control, sink)?,
            aylis_project::WindowsInstaller::Inno => win_installer::inno(project, &dest, &extra, out, control, sink)?,
        };
        return Ok(Published { main, dir: out.to_path_buf() });
    }
    Ok(Published { main: dest, dir: out.to_path_buf() })
}

/// Um GUID estável derivado do texto (o UpgradeCode: o mesmo em todas as versões, para o
/// Windows trocar a versão antiga pela nova).
fn stable_guid(text: &str) -> String {
    // FNV-1a de 128 bits feito com duas metades de 64.
    let hash = |seed: u64| text.bytes().fold(seed, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3));
    let (a, b) = (hash(0xcbf29ce484222325), hash(0x84222325cbf29ce4));
    let hex = format!("{a:016X}{b:016X}");
    format!("{}-{}-4{}-8{}-{}", &hex[0..8], &hex[8..12], &hex[13..16], &hex[17..20], &hex[20..32])
}

/// O instalador do Windows (`.msi`, pelo WiX): instala em Arquivos de Programas, com atalho
/// no menu Iniciar e desinstalação em Aplicativos; uma versão nova substitui a anterior.
fn windows_installer(project: &Project, exe_path: &Path, extra: &[PathBuf], out: &Path, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    let wix = wix().ok_or("WiX não encontrado (Ferramentas → Verificar ambiente instala; ou dotnet tool install --global wix --version 5.0.2)")?;
    sink(Event::Step("Montando o instalador (.msi)…".into()));
    let app = xml_escape(project.app_name());
    let publisher = xml_escape(if project.publish.publisher.is_empty() { project.app_name() } else { &project.publish.publisher });
    // O MSI quer a versão com 4 partes, cada uma até 65535.
    let mut parts: Vec<u32> = project.version().split('.').map(|p| p.parse::<u32>().unwrap_or(0).min(65535)).collect();
    parts.resize(4, 0);
    let version = parts.iter().map(u32::to_string).collect::<Vec<_>>().join(".");
    let exe_name = exe_path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let upgrade = stable_guid(&project.identifier());
    // Os arquivos ao lado do executável (o compilador DXC do traçado de raios).
    let extra_files: String = extra.iter().enumerate().map(|(i, p)| format!("\n          <File Id=\"Extra{i}\" Source=\"{}\" />", xml_escape(&p.display().to_string()))).collect();
    let icon = if project.publish.icon.is_empty() { String::new() } else { format!("\n    <Property Id=\"ARPPRODUCTICON\" Value=\"AppIcon\" />\n    <Icon Id=\"AppIcon\" SourceFile=\"{}\" />", xml_escape(&exe_path.display().to_string())) };
    let wxs = format!(
        r#"<!-- Gerado pelo aylis (Publicar → Windows → instalador). -->
<Wix xmlns="http://wixtoolset.org/schemas/v4/wxs">
  <Package Name="{app}" Manufacturer="{publisher}" Version="{version}" UpgradeCode="{upgrade}" Scope="perMachine">
    <MajorUpgrade DowngradeErrorMessage="Uma versão mais nova de {app} já está instalada." />
    <MediaTemplate EmbedCab="yes" />{icon}
    <StandardDirectory Id="ProgramFiles64Folder">
      <Directory Id="INSTALLFOLDER" Name="{app}">
        <Component Id="MainExe" Guid="*">
          <File Id="AppExe" Source="{source}" KeyPath="yes" />{extra_files}
        </Component>
      </Directory>
    </StandardDirectory>
    <StandardDirectory Id="ProgramMenuFolder">
      <Component Id="StartMenu" Guid="*">
        <Shortcut Id="AppShortcut" Name="{app}" Target="[INSTALLFOLDER]{exe_name}" WorkingDirectory="INSTALLFOLDER" />
        <RegistryValue Root="HKCU" Key="Software\aylis\{app}" Name="atalho" Type="integer" Value="1" KeyPath="yes" />
      </Component>
    </StandardDirectory>
    <Feature Id="Main" Title="{app}">
      <ComponentRef Id="MainExe" />
      <ComponentRef Id="StartMenu" />
    </Feature>
  </Package>
</Wix>
"#,
        source = xml_escape(&exe_path.display().to_string()),
    );
    let wxs_path = out.join("instalador.wxs");
    std::fs::write(&wxs_path, wxs).map_err(|e| e.to_string())?;
    let msi = out.join(format!("{}-{}.msi", file_name(project.app_name()), project.version()));
    run_tool(Command::new(&wix).arg("build").arg(&wxs_path).arg("-arch").arg("x64").arg("-o").arg(&msi), control, sink, LineKind::Tool)?;
    let _ = std::fs::remove_file(&wxs_path);
    let _ = std::fs::remove_file(msi.with_extension("wixpdb"));
    sink(Event::Line(format!("{} ({})", msi.display(), size(&msi)), LineKind::Tool));
    Ok(msi)
}

fn desktop_python(project: &Project, options: &Options, out: &Path, sink: Sink) -> Result<Published, String> {
    sink(Event::Step(format!("Montando {} (Python)…", project.name)));
    let host = options.aylis_host.clone().filter(|h| h.is_file()).ok_or("aylis-host não encontrado (compile com: cargo build -p aylis-host --release)")?;
    let app = out.join(file_name(project.app_name()));
    std::fs::create_dir_all(&app).map_err(|e| e.to_string())?;
    let host_name = exe("aylis-host");
    std::fs::copy(&host, app.join(&host_name)).map_err(|e| format!("aylis-host: {e}"))?;
    let project_file = project.path.file_name().ok_or("nome do .aylisproj")?;
    std::fs::copy(&project.path, app.join(project_file)).map_err(|e| e.to_string())?;
    for sub in aylis_build::PACKED_DIRS {
        let from = project.dir().join(sub);
        if from.is_dir() {
            copy_dir(&from, &app.join(sub))?;
        }
    }
    // O código Python (da pasta da linguagem) vai para `src/` ao lado do projeto.
    copy_dir(&project.code_dir().join("src"), &app.join("src"))?;
    let _ = std::fs::remove_dir_all(app.join("src/__pycache__"));
    let _ = std::fs::remove_dir_all(app.join("src/forms/__pycache__"));
    let project_file = project_file.to_string_lossy();
    let launcher = if cfg!(windows) {
        let path = app.join(format!("{}.cmd", file_name(project.app_name())));
        let text = format!(
            "@echo off\r\nrem {} — precisa do Python 3.8+ instalado.\r\ncd /d \"%~dp0\"\r\nstart \"\" \"%~dp0{host_name}\" --project \"{project_file}\" -- python src\\main.py\r\n",
            project.app_name()
        );
        std::fs::write(&path, text).map_err(|e| e.to_string())?;
        path
    } else {
        let path = app.join(file_name(project.app_name()));
        let text = format!("#!/bin/sh\n# {} — precisa do Python 3.8+ instalado.\ncd \"$(dirname \"$0\")\"\nexec ./{host_name} --project \"{project_file}\" -- python3 src/main.py\n", project.app_name());
        std::fs::write(&path, text).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755));
        }
        path
    };
    sink(Event::Line(format!("{} (abra este atalho; a máquina precisa do Python)", launcher.display()), LineKind::Tool));
    Ok(Published { main: launcher, dir: app })
}

/// TypeScript, C#, Java, Go, C++ e Free Pascal: o aylis-host, o projeto e o backend
/// compilado (otimizado) numa pasta, com um atalho que abre tudo.
fn desktop_host(project: &Project, options: &Options, out: &Path, control: &Control, sink: Sink) -> Result<Published, String> {
    let host = options.aylis_host.clone().filter(|h| h.is_file()).ok_or("aylis-host não encontrado (compile com: cargo build -p aylis-host --release)")?;
    aylis_project::codegen::generate(project)?;
    let tc = crate::hostlang::find(project.language)?;
    let app = out.join(file_name(project.app_name()));
    std::fs::create_dir_all(&app).map_err(|e| e.to_string())?;
    let host_name = exe("aylis-host");
    std::fs::copy(&host, app.join(&host_name)).map_err(|e| format!("aylis-host: {e}"))?;
    let project_file = project.path.file_name().ok_or("nome do .aylisproj")?;
    ray_tracing_dlls(project, &app, sink)?;
    std::fs::copy(&project.path, app.join(project_file)).map_err(|e| e.to_string())?;
    for sub in aylis_build::PACKED_DIRS {
        let from = project.dir().join(sub);
        if from.is_dir() {
            copy_dir(&from, &app.join(sub))?;
        }
    }
    // TypeScript roda das fontes; as outras levam o backend compilado.
    let backend_dir = app.join("backend");
    let mut command = crate::hostlang::build(project, &tc, &backend_dir, true, control, sink)?;
    if project.language == Language::TypeScript {
        copy_dir(&project.code_dir().join("src"), &app.join("src"))?;
        command[0] = tc.program.file_stem().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "bun".into());
    }
    // Caminhos relativos à pasta publicada.
    let base = format!("{}{}", app.display(), std::path::MAIN_SEPARATOR);
    let rel: Vec<String> = command.iter().map(|a| a.replace(&base, "")).collect();
    let needs = match project.language {
        Language::TypeScript => "o Bun ou o Node 22.6+",
        Language::CSharp => "o .NET 8",
        Language::Java => "o Java 17+",
        _ => "",
    };
    let quoted: Vec<String> = rel.iter().map(|a| if a.contains(' ') { format!("\"{a}\"") } else { a.clone() }).collect();
    let project_file = project_file.to_string_lossy();
    let note = if needs.is_empty() { String::new() } else { format!(" — precisa d{needs} instalado") };
    let launcher = if cfg!(windows) {
        let path = app.join(format!("{}.cmd", file_name(project.app_name())));
        let text = format!(
            "@echo off\r\nrem {}{note}.\r\ncd /d \"%~dp0\"\r\nstart \"\" \"%~dp0{host_name}\" --project \"{project_file}\" -- {}\r\n",
            project.app_name(),
            quoted.join(" ")
        );
        std::fs::write(&path, text).map_err(|e| e.to_string())?;
        path
    } else {
        let path = app.join(file_name(project.app_name()));
        let text = format!("#!/bin/sh\n# {}{note}.\ncd \"$(dirname \"$0\")\"\nexec ./{host_name} --project \"{project_file}\" -- {}\n", project.app_name(), quoted.join(" "));
        std::fs::write(&path, text).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755));
        }
        path
    };
    sink(Event::Line(format!("{} (abra este atalho)", launcher.display()), LineKind::Tool));
    Ok(Published { main: launcher, dir: app })
}

// ---------------------------------------------------------------------------
// Android

/// Arquivos do Android usados na compilação.
struct AndroidTools {
    ndk: PathBuf,
    build_tools: PathBuf,
    platform_jar: PathBuf,
    java: PathBuf,
}

fn android_tools() -> Result<AndroidTools, String> {
    let sdk = env::android_sdk().ok_or("Android SDK não encontrado (Ferramentas → Verificar ambiente)")?;
    Ok(AndroidTools {
        ndk: env::android_ndk(&sdk).ok_or("NDK não encontrado (Ferramentas → Verificar ambiente)")?,
        build_tools: env::android_build_tools(&sdk).ok_or("build tools do Android não encontradas")?,
        platform_jar: env::android_platform(&sdk).ok_or("plataforma do Android não encontrada")?.join("android.jar"),
        java: env::java_bin().ok_or("Java (JDK 17+) não encontrado")?,
    })
}

/// O ABI do primeiro aparelho conectado (para o Executar compilar só o necessário).
pub fn device_abi(serial: Option<&str>) -> Option<String> {
    let sdk = env::android_sdk()?;
    let adb = env::adb(&sdk).display().to_string();
    let mut args = Vec::new();
    if let Some(s) = serial {
        args.extend(["-s", s]);
    }
    args.extend(["shell", "getprop", "ro.product.cpu.abi"]);
    env::output(&adb, &args).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

fn android(project: &Project, options: &Options, out: &Path, control: &Control, sink: Sink) -> Result<Published, String> {
    let tools = android_tools()?;
    let abis: Vec<(&str, &str)> = env::ANDROID_ABIS
        .iter()
        .copied()
        .filter(|(abi, _)| options.abis.is_empty() || options.abis.iter().any(|a| a == abi))
        .collect();
    if abis.is_empty() {
        return Err(format!("arquitetura não suportada: {}", options.abis.join(", ")));
    }
    let package = project.package_name();
    let work = out.join("build");
    let _ = std::fs::remove_dir_all(&work);
    std::fs::create_dir_all(&work).map_err(|e| e.to_string())?;

    // 1. A biblioteca nativa de cada arquitetura.
    let mut libs = Vec::new();
    for (abi, triple) in &abis {
        control.check()?;
        sink(Event::Step(format!("Compilando para Android {abi}…")));
        let mut args = vec!["ndk", "-t", abi, "-P", "26", "rustc", "--lib", "--crate-type", "cdylib"];
        if !options.debug {
            args.push("--release");
        }
        let features = options.features.join(",");
        if !features.is_empty() {
            args.extend(["--features", features.as_str()]);
        }
        let crate_dir = options.crate_dir.clone().unwrap_or_else(|| project.code_dir());
        let built = cargo("cargo", &args, &crate_dir, &[("ANDROID_NDK_HOME", tools.ndk.as_path())], control, sink)?;
        let so = built
            .artifacts
            .iter()
            .flat_map(|a| a.filenames.iter())
            .find(|f| f.extension().is_some_and(|e| e == "so") && f.to_string_lossy().contains(triple))
            .cloned()
            .unwrap_or_else(|| crate::shared_target_dir().join(triple).join(profile(options)).join(format!("lib{package}.so")));
        if !so.is_file() {
            return Err(format!("a biblioteca {} não foi gerada", so.display()));
        }
        let dest = work.join("lib").join(abi).join(format!("lib{package}.so"));
        std::fs::create_dir_all(dest.parent().expect("tem pai")).map_err(|e| e.to_string())?;
        std::fs::copy(&so, &dest).map_err(|e| e.to_string())?;
        libs.push(dest);
    }

    // 2. Manifesto e recursos (o ícone).
    sink(Event::Step("Montando o APK…".into()));
    let manifest = work.join("AndroidManifest.xml");
    let has_icon = android_icon(project, &work)?;
    std::fs::write(&manifest, android_manifest(project, &package, has_icon)?).map_err(|e| e.to_string())?;
    let bt = |name: &str| tools.build_tools.join(name);
    let mut link = Command::new(bt(&exe("aapt2")));
    let unsigned = work.join("unsigned.apk");
    link.arg("link").arg("-o").arg(&unsigned).arg("--manifest").arg(&manifest).arg("-I").arg(&tools.platform_jar);
    // Desenvolvimento (`--debug` ou o `dev` do Executar): depurável, para o `run-as` do
    // `aylis-tool debug-android`.
    if options.debug || options.features.iter().any(|f| f == "dev") {
        link.arg("--debug-mode");
    }
    if has_icon {
        let compiled = work.join("res.zip");
        run_tool(Command::new(bt(&exe("aapt2"))).arg("compile").arg("--dir").arg(work.join("res")).arg("-o").arg(&compiled), control, sink, LineKind::Tool)?;
        link.arg("-R").arg(&compiled).arg("--auto-add-overlay");
    }
    run_tool(&mut link, control, sink, LineKind::Tool)?;
    // A `AylisActivity` (teclado completo) vira o classes.dex.
    android_activity_dex(&tools, &work, control, sink)?;
    // As bibliotecas e o código Java entram no APK com o `jar` do JDK (sem o manifesto do
    // Java).
    run_tool(
        Command::new(tools.java.join(exe("jar")))
            .arg("--update")
            .arg("--file")
            .arg(&unsigned)
            .arg("--no-manifest")
            .arg("-C")
            .arg(&work)
            .arg("lib")
            .arg("-C")
            .arg(work.join("dex"))
            .arg("classes.dex"),
        control,
        sink,
        LineKind::Tool,
    )?;

    // A chave: a do desenvolvedor, ou a de depuração.
    let (keystore, alias, store_pass, key_pass) = match &options.signing {
        Some(s) => {
            if !s.keystore.is_file() {
                return Err(format!("chave de assinatura não encontrada: {}", s.keystore.display()));
            }
            (s.keystore.clone(), s.alias.clone(), s.store_password.clone(), s.key_password.clone())
        }
        None => (debug_keystore(&tools, control, sink)?, "androiddebugkey".into(), "android".into(), "android".into()),
    };
    if options.android_bundle {
        return android_bundle(project, &tools, &work, out, has_icon, &manifest, (&keystore, &alias, &store_pass, &key_pass), options.signing.is_some(), control, sink);
    }

    // 3. Alinhar e assinar.
    let aligned = work.join("aligned.apk");
    run_tool(Command::new(bt(&exe("zipalign"))).args(["-p", "-f", "4"]).arg(&unsigned).arg(&aligned), control, sink, LineKind::Tool)?;
    let apk = out.join(format!("{}-{}.apk", file_name(project.app_name()), project.version()));
    let apksigner = bt(if cfg!(windows) { "apksigner.bat" } else { "apksigner" });
    // As senhas pelo ambiente do processo (não aparecem na linha de comando).
    run_tool(
        Command::new(apksigner)
            .args(["sign", "--ks"])
            .arg(&keystore)
            .args(["--ks-key-alias", &alias, "--ks-pass", "env:AYLIS_SIGN_STORE", "--key-pass", "env:AYLIS_SIGN_KEY", "--out"])
            .arg(&apk)
            .arg(&aligned)
            .env("AYLIS_SIGN_STORE", &store_pass)
            .env("AYLIS_SIGN_KEY", &key_pass)
            .env("JAVA_HOME", tools.java.parent().unwrap_or(&tools.java)),
        control,
        sink,
        LineKind::Tool,
    )?;
    if options.signing.is_none() {
        sink(Event::Line("Assinado com a chave de depuração: para a Play Store, configure a chave do desenvolvedor (Projeto → Opções).".into(), LineKind::Tool));
    }
    let _ = std::fs::remove_dir_all(&work);
    let _ = std::fs::remove_file(out.join(format!("{}.idsig", apk.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default())));
    let archs = abis.iter().map(|(a, _)| *a).collect::<Vec<_>>().join(", ");
    let key = if options.signing.is_some() { "assinado com a chave do desenvolvedor" } else { "assinado com a chave de depuração" };
    sink(Event::Line(format!("{} ({}, {archs}; {key})", apk.display(), size(&apk)), LineKind::Tool));
    Ok(Published { main: apk, dir: out.to_path_buf() })
}

/// O código Java do aylis no Android: a `dev.aylis.AylisActivity`, a NativeActivity com o
/// teclado virtual completo (sugestões, correção, emoji). Compilada com o `javac` do JDK e
/// convertida com o `d8` das build tools em `work/dex/classes.dex`.
const AYLIS_ACTIVITY: &str = include_str!("../android/AylisActivity.java");

fn android_activity_dex(tools: &AndroidTools, work: &Path, control: &Control, sink: Sink) -> Result<(), String> {
    let src = work.join("java/dev/aylis/AylisActivity.java");
    let classes = work.join("classes");
    let dex = work.join("dex");
    for dir in [src.parent().expect("tem pai"), &classes, &dex] {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(&src, AYLIS_ACTIVITY).map_err(|e| e.to_string())?;
    run_tool(
        Command::new(tools.java.join(exe("javac")))
            .args(["-source", "8", "-target", "8", "-nowarn", "-encoding", "UTF-8", "-bootclasspath"])
            .arg(&tools.platform_jar)
            .arg("-d")
            .arg(&classes)
            .arg(&src),
        control,
        sink,
        LineKind::Tool,
    )?;
    let class_files: Vec<PathBuf> = std::fs::read_dir(classes.join("dev/aylis"))
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "class"))
        .collect();
    let d8 = tools.build_tools.join(if cfg!(windows) { "d8.bat" } else { "d8" });
    run_tool(
        Command::new(d8)
            .args(["--min-api", "26", "--lib"])
            .arg(&tools.platform_jar)
            .arg("--output")
            .arg(&dex)
            .args(&class_files)
            .env("JAVA_HOME", tools.java.parent().unwrap_or(&tools.java)),
        control,
        sink,
        LineKind::Tool,
    )
}

/// O que os componentes `Permissions` e `Biometrics` dos forms pedem ao sistema.
#[derive(Default)]
struct DeviceNeeds {
    permissions: Vec<aylis_core::platform::Permission>,
    /// O texto que explica cada uma (`reasons`): o iOS exige.
    reasons: Vec<(aylis_core::platform::Permission, String)>,
    /// Usa biometria, com o texto (`reason` do `Biometrics`).
    biometrics: Option<String>,
    /// Um `AudioMixer` abre entradas do sistema (`device:`): pede o microfone.
    mixer_inputs: bool,
}

/// Lê dos forms (e subpastas) os `Permissions` (`permissions`, `reasons`) e os `Biometrics`.
/// O `BackgroundService` dos forms: o tipo do serviço (`kind`) e o `keep_alive`.
fn background_service(forms: &Path) -> Option<(String, bool)> {
    fn walk(value: &serde_json::Value) -> Option<(String, bool)> {
        match value {
            serde_json::Value::Object(map) if map.get("class").and_then(|c| c.as_str()) == Some("BackgroundService") => Some((
                map.get("kind").and_then(|k| k.as_str()).filter(|k| !k.is_empty()).unwrap_or("dataSync").to_string(),
                map.get("keep_alive").and_then(|k| k.as_bool()).unwrap_or(false),
            )),
            serde_json::Value::Object(map) => map.values().find_map(walk),
            serde_json::Value::Array(items) => items.iter().find_map(walk),
            _ => None,
        }
    }
    std::fs::read_dir(forms).into_iter().flatten().flatten().find_map(|e| {
        std::fs::read_to_string(e.path()).ok().and_then(|t| serde_json::from_str::<serde_json::Value>(t.trim_start_matches('\u{feff}')).ok()).and_then(|v| walk(&v))
    })
}

/// Algum form usa o som dos outros apps do Android (`loopback:Android`).
fn uses_android_loopback(forms: &Path) -> bool {
    std::fs::read_dir(forms).into_iter().flatten().flatten().any(|e| std::fs::read_to_string(e.path()).is_ok_and(|t| t.contains("loopback:Android")))
}

/// Algum componente da classe `class` nos forms tem `prop` ligada.
fn component_flag(forms: &Path, class: &str, prop: &str) -> bool {
    fn walk(value: &serde_json::Value, class: &str, prop: &str) -> bool {
        match value {
            serde_json::Value::Object(map) => {
                (map.get("class").and_then(|c| c.as_str()) == Some(class) && map.get(prop).and_then(|v| v.as_bool()) == Some(true))
                    || map.values().any(|v| walk(v, class, prop))
            }
            serde_json::Value::Array(items) => items.iter().any(|v| walk(v, class, prop)),
            _ => false,
        }
    }
    std::fs::read_dir(forms).into_iter().flatten().flatten().any(|e| {
        std::fs::read_to_string(e.path()).ok().and_then(|t| serde_json::from_str::<serde_json::Value>(t.trim_start_matches('\u{feff}')).ok()).is_some_and(|v| walk(&v, class, prop))
    })
}

/// A permissão do Android 14+ para o tipo do serviço em primeiro plano.
fn foreground_permission(kind: &str) -> &'static str {
    match kind {
        "location" => "android.permission.FOREGROUND_SERVICE_LOCATION",
        "mediaPlayback" => "android.permission.FOREGROUND_SERVICE_MEDIA_PLAYBACK",
        "connectedDevice" => "android.permission.FOREGROUND_SERVICE_CONNECTED_DEVICE",
        "microphone" => "android.permission.FOREGROUND_SERVICE_MICROPHONE",
        "camera" => "android.permission.FOREGROUND_SERVICE_CAMERA",
        _ => "android.permission.FOREGROUND_SERVICE_DATA_SYNC",
    }
}

/// Algum form trata o evento `event` (a chave dele, com um handler, em qualquer componente).
fn forms_use_event(forms: &Path, event: &str) -> bool {
    fn walk(value: &serde_json::Value, event: &str) -> bool {
        match value {
            serde_json::Value::Object(map) => map.get(event).and_then(|v| v.as_str()).is_some_and(|h| !h.is_empty()) || map.values().any(|v| walk(v, event)),
            serde_json::Value::Array(items) => items.iter().any(|v| walk(v, event)),
            _ => false,
        }
    }
    std::fs::read_dir(forms).into_iter().flatten().flatten().any(|e| {
        std::fs::read_to_string(e.path()).ok().and_then(|t| serde_json::from_str::<serde_json::Value>(t.trim_start_matches('\u{feff}')).ok()).is_some_and(|v| walk(&v, event))
    })
}

fn device_needs(forms: &Path) -> DeviceNeeds {
    use aylis_core::platform::Permission;
    fn walk(value: &serde_json::Value, needs: &mut DeviceNeeds) {
        match value {
            serde_json::Value::Object(map) => {
                let class = map.get("class").and_then(|c| c.as_str()).unwrap_or("");
                let text = |key: &str| map.get(key).and_then(|v| v.as_str()).unwrap_or("").to_string();
                if class == "Permissions" {
                    for p in Permission::parse_list(&text("permissions")) {
                        if !needs.permissions.contains(&p) {
                            needs.permissions.push(p);
                        }
                    }
                    for (name, why) in text("reasons").split(';').filter_map(|pair| pair.split_once('=')) {
                        if let Some(p) = Permission::parse(name) {
                            needs.reasons.push((p, why.trim().to_string()));
                        }
                    }
                }
                if class == "AudioMixer" && text("sources").contains("device:") {
                    needs.mixer_inputs = true;
                }
                // O componente Bluetooth: as permissões do Android e a chave do Info.plist.
                if class == "Bluetooth" && !needs.permissions.contains(&aylis_core::platform::Permission::Bluetooth) {
                    needs.permissions.push(aylis_core::platform::Permission::Bluetooth);
                }
                if class == "Biometrics" {
                    let reason = text("reason");
                    if needs.biometrics.as_ref().is_none_or(|r| r.is_empty()) {
                        needs.biometrics = Some(reason);
                    }
                }
                map.values().for_each(|v| walk(v, needs));
            }
            serde_json::Value::Array(items) => items.iter().for_each(|v| walk(v, needs)),
            _ => {}
        }
    }
    fn files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                files(&p, out);
            } else if p.extension().is_some_and(|x| x == "json") {
                out.push(p);
            }
        }
    }
    let mut needs = DeviceNeeds::default();
    let mut list = Vec::new();
    files(forms, &mut list);
    for f in list {
        if let Ok(Ok(value)) = std::fs::read_to_string(&f).map(|t| serde_json::from_str::<serde_json::Value>(t.trim_start_matches('\u{feff}'))) {
            walk(&value, &mut needs);
        }
    }
    needs
}

/// As permissões do Android de cada uma (as mesmas que o app pede em tempo de execução).
fn android_permission_names(p: aylis_core::platform::Permission) -> &'static [&'static str] {
    use aylis_core::platform::Permission::*;
    match p {
        Camera => &["android.permission.CAMERA"],
        Microphone => &["android.permission.RECORD_AUDIO"],
        Location => &["android.permission.ACCESS_FINE_LOCATION", "android.permission.ACCESS_COARSE_LOCATION"],
        Notifications => &["android.permission.POST_NOTIFICATIONS"],
        Photos => &["android.permission.READ_MEDIA_IMAGES", "android.permission.READ_MEDIA_VIDEO", "android.permission.READ_EXTERNAL_STORAGE\" android:maxSdkVersion=\"32"],
        Contacts => &["android.permission.READ_CONTACTS"],
        Calendar => &["android.permission.READ_CALENDAR", "android.permission.WRITE_CALENDAR"],
        Bluetooth => &["android.permission.BLUETOOTH_CONNECT", "android.permission.BLUETOOTH_SCAN"],
    }
}

/// As chaves do Info.plist que o iOS exige de cada uma (o texto aparece na pergunta).
fn ios_usage_keys(p: aylis_core::platform::Permission) -> &'static [&'static str] {
    use aylis_core::platform::Permission::*;
    match p {
        Camera => &["NSCameraUsageDescription"],
        Microphone => &["NSMicrophoneUsageDescription"],
        Location => &["NSLocationWhenInUseUsageDescription"],
        Notifications => &[],
        Photos => &["NSPhotoLibraryUsageDescription"],
        Contacts => &["NSContactsUsageDescription"],
        Calendar => &["NSCalendarsFullAccessUsageDescription", "NSCalendarsUsageDescription"],
        Bluetooth => &["NSBluetoothAlwaysUsageDescription"],
    }
}

/// As entradas do Info.plist das permissões e da biometria (sem texto, um genérico: sem a
/// chave o iOS fecha o app ao perguntar).
fn ios_usage_plist(needs: &DeviceNeeds, saves: bool) -> String {
    let mut out = String::new();
    // O SaveDialog grava na pasta Documentos do app: o app Arquivos passa a mostrá-la.
    if saves {
        out.push_str("  <key>UIFileSharingEnabled</key><true/>\n  <key>LSSupportsOpeningDocumentsInPlace</key><true/>\n");
    }
    let mut add = |key: &str, text: &str| {
        let text = if text.trim().is_empty() { "Usado pelos recursos do aplicativo." } else { text };
        out.push_str(&format!("  <key>{key}</key><string>{}</string>\n", xml_escape(text)));
    };
    for &p in &needs.permissions {
        let reason = needs.reasons.iter().find(|(q, _)| *q == p).map(|(_, r)| r.as_str()).unwrap_or("");
        for key in ios_usage_keys(p) {
            add(key, reason);
        }
    }
    if needs.mixer_inputs && !needs.permissions.contains(&aylis_core::platform::Permission::Microphone) {
        add("NSMicrophoneUsageDescription", "");
    }
    if let Some(reason) = &needs.biometrics {
        add("NSFaceIDUsageDescription", reason);
    }
    out
}

/// As permissões que os componentes usados nos forms pedem (só as necessárias: a loja
/// cobra justificativa de cada uma).

/// As permissões que os componentes do projeto pedem, com o motivo (o que a aba Android
/// mostra marcado e travado).
pub fn android_required_permissions(project: &Project) -> Vec<(String, String)> {
    let forms = project.dir().join("forms");
    let classes = aylis_build::used_classes(&forms);
    let needs = device_needs(&forms);
    let mut out: Vec<(String, String)> = vec![("android.permission.INTERNET".into(), "o Aylis (rede e canal de desenvolvimento)".into())];
    let mut add = |name: &str, why: String| {
        // O nome sem os atributos a mais (`" android:maxSdkVersion="30`): uma permissão só uma vez.
        let base = |n: &str| n.split('"').next().unwrap_or(n).to_string();
        if !out.iter().any(|(n, _)| base(n) == base(name)) {
            out.push((name.to_string(), why));
        }
    };
    for &p in &needs.permissions {
        for name in android_permission_names(p) {
            add(name, format!("componente Permissions ({p:?})"));
        }
    }
    if needs.mixer_inputs {
        add("android.permission.RECORD_AUDIO", "componente AudioMixer (entradas do sistema)".into());
    }
    if needs.biometrics.is_some() {
        add("android.permission.USE_BIOMETRIC", "componente Biometrics".into());
    }
    for (class, name) in [
        ("CameraView", "android.permission.CAMERA"),
        ("Microphone", "android.permission.RECORD_AUDIO"),
        ("Notification", "android.permission.POST_NOTIFICATIONS"),
        ("Location", "android.permission.ACCESS_FINE_LOCATION"),
        ("Location", "android.permission.ACCESS_COARSE_LOCATION"),
        ("Network", "android.permission.ACCESS_NETWORK_STATE"),
        ("Vibration", "android.permission.VIBRATE"),
        ("Nfc", "android.permission.NFC"),
        // O push do FCM: mostrar a notificação e receber do Play Services.
        ("PushNotifications", "android.permission.POST_NOTIFICATIONS"),
        ("PushNotifications", "com.google.android.c2dm.permission.RECEIVE"),
        // As ações do aparelho: notificar e, no WakeAppAction, abrir o app em tela cheia.
        ("NotifyAction", "android.permission.POST_NOTIFICATIONS"),
        ("WakeAppAction", "android.permission.POST_NOTIFICATIONS"),
        ("WakeAppAction", "android.permission.USE_FULL_SCREEN_INTENT"),
        ("WakeAppAction", "android.permission.VIBRATE"),
        // O Android 12+ pede as duas do Bluetooth; antes, a localização (procurar) e as antigas.
        ("Bluetooth", "android.permission.BLUETOOTH_SCAN"),
        ("Bluetooth", "android.permission.BLUETOOTH_CONNECT"),
        ("Bluetooth", "android.permission.ACCESS_FINE_LOCATION\" android:maxSdkVersion=\"30"),
        ("Bluetooth", "android.permission.BLUETOOTH\" android:maxSdkVersion=\"30"),
        ("Bluetooth", "android.permission.BLUETOOTH_ADMIN\" android:maxSdkVersion=\"30"),
    ] {
        if classes.iter().any(|c| c == class) {
            add(name, format!("componente {class}"));
        }
    }
    // As ligações: o que cada opção do Phone pede (a Play Store restringe, a aba Android avisa).
    let phone = |prop: &str| component_flag(&forms, "Phone", prop);
    if phone("direct_call") || phone("dialer_app") {
        add("android.permission.CALL_PHONE", "componente Phone (ligar direto)".into());
    }
    if phone("watch_state") || phone("dialer_app") {
        add("android.permission.READ_PHONE_STATE", "componente Phone (estado das chamadas)".into());
        add("android.permission.READ_CALL_LOG", "componente Phone (o número e o histórico)".into());
    }
    if uses_android_loopback(&forms) {
        add("android.permission.RECORD_AUDIO", "o som dos outros apps (loopback:Android)".into());
        add("android.permission.FOREGROUND_SERVICE", "o som dos outros apps (loopback:Android)".into());
        add("android.permission.FOREGROUND_SERVICE_MEDIA_PROJECTION", "o som dos outros apps (loopback:Android)".into());
    }
    if component_flag(&forms, "Notification", "full_screen") {
        add("android.permission.USE_FULL_SCREEN_INTENT", "componente Notification (tela cheia)".into());
    }
    if let Some((kind, keep_alive)) = background_service(&forms) {
        let why = || "componente BackgroundService".to_string();
        add("android.permission.FOREGROUND_SERVICE", why());
        add(foreground_permission(&kind), why());
        add("android.permission.POST_NOTIFICATIONS", why());
        if keep_alive {
            add("android.permission.RECEIVE_BOOT_COMPLETED", why());
            add("android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS", why());
        }
    }
    out
}

/// As permissões que a Play Store restringe (só aceita com justificativa ou em apps de
/// certas categorias): a aba Android avisa.
pub const ANDROID_RESTRICTED: [&str; 12] = [
    "android.permission.CALL_PHONE",
    "android.permission.READ_PHONE_STATE",
    "android.permission.READ_CALL_LOG",
    "android.permission.WRITE_CALL_LOG",
    "android.permission.READ_SMS",
    "android.permission.SEND_SMS",
    "android.permission.ACCESS_BACKGROUND_LOCATION",
    "android.permission.MANAGE_EXTERNAL_STORAGE",
    "android.permission.QUERY_ALL_PACKAGES",
    "android.permission.REQUEST_INSTALL_PACKAGES",
    "android.permission.SYSTEM_ALERT_WINDOW",
    "android.permission.USE_FULL_SCREEN_INTENT",
];

/// O arquivo do projeto com o XML a mais do manifesto (mesclado no gerado).
pub const ANDROID_MANIFEST_EXTRA: &str = "android/manifest-extra.xml";

/// O manifesto final do Android (o mesmo do publicar), para a aba Android mostrar.
pub fn android_manifest_preview(project: &Project) -> Result<String, String> {
    let has_icon = !project.publish.icon.is_empty();
    android_manifest(project, &project.package_name(), has_icon)
}

fn android_manifest(project: &Project, package: &str, icon: bool) -> Result<String, String> {
    let a = &project.publish.android;
    let version_code: u32 = if a.version_code > 0 {
        a.version_code
    } else {
        // 1.2.3 → 10203
        let mut parts = project.version().split('.').map(|p| p.parse::<u32>().unwrap_or(0));
        let (x, y, z) = (parts.next().unwrap_or(1), parts.next().unwrap_or(0), parts.next().unwrap_or(0));
        (x * 10000 + y * 100 + z).max(1)
    };
    let (min_sdk, target_sdk) = (if a.min_sdk > 0 { a.min_sdk } else { 26 }, if a.target_sdk > 0 { a.target_sdk } else { 34 });
    let mut permissions = String::new();
    let mut declared: Vec<String> = Vec::new();
    for (name, _) in android_required_permissions(project).into_iter().chain(a.permissions.iter().map(|p| (p.clone(), String::new()))) {
        let full = if name.contains('.') { name } else { format!("android.permission.{name}") };
        if !declared.contains(&full) {
            permissions.push_str(&format!("    <uses-permission android:name=\"{full}\" />\n"));
            declared.push(full);
        }
    }
    let forms = project.dir().join("forms");
    let camera = aylis_build::used_classes(&forms).iter().any(|c| c == "CameraView") || declared.iter().any(|d| d == "android.permission.CAMERA");
    let mut features: Vec<(String, bool)> = a.features.clone();
    if camera && !features.iter().any(|(f, _)| f.starts_with("android.hardware.camera")) {
        features.push(("android.hardware.camera.any".into(), false));
    }
    for (name, required) in &features {
        permissions.push_str(&format!("    <uses-feature android:name=\"{}\" android:required=\"{required}\" />\n", xml_escape(name)));
    }
    if a.tablet_only {
        permissions.push_str("    <supports-screens android:smallScreens=\"false\" android:normalScreens=\"false\" android:largeScreens=\"true\" android:xlargeScreens=\"true\" />\n");
    }
    let icon = if icon { "\n        android:icon=\"@mipmap/ic_launcher\"" } else { "" };
    let theme = if a.fullscreen { "@android:style/Theme.DeviceDefault.NoActionBar.Fullscreen" } else { "@android:style/Theme.DeviceDefault.NoActionBar" };
    let orientation = if a.orientation.is_empty() { String::new() } else { format!("\n            android:screenOrientation=\"{}\"", xml_escape(&a.orientation)) };
    let keep_on = if a.keep_screen_on { "\n            android:keepScreenOn=\"true\"" } else { "" };
    // O AppLink: compartilhar arquivos (o provedor deles) e, com o `on_shared` num form, o app
    // na lista de compartilhar dos outros.
    let app_link = aylis_build::used_classes(&forms).iter().any(|c| c == "AppLink");
    let receives_shares = app_link && forms_use_event(&forms, "on_shared");
    let mut provider = if app_link {
        format!(
            "        <provider android:name=\"dev.aylis.AylisActivity$Files\" android:authorities=\"{}.aylis.files\" android:exported=\"false\" android:grantUriPermissions=\"true\" />\n",
            project.identifier()
        )
    } else {
        String::new()
    };
    // O app de telefone: o InCallService (o sistema entrega as chamadas a ele) e o discador
    // (sem o DIAL, o Android não deixa o app virar o discador padrão).
    let mut dialer = String::new();
    if component_flag(&forms, "Phone", "dialer_app") {
        provider.push_str("        <service android:name=\"dev.aylis.AylisActivity$Calls\" android:permission=\"android.permission.BIND_INCALL_SERVICE\" android:exported=\"true\">\n            <meta-data android:name=\"android.telecom.IN_CALL_SERVICE_UI\" android:value=\"true\" />\n            <intent-filter>\n                <action android:name=\"android.telecom.InCallService\" />\n            </intent-filter>\n        </service>\n");
        dialer.push_str("            <intent-filter>\n                <action android:name=\"android.intent.action.DIAL\" />\n                <category android:name=\"android.intent.category.DEFAULT\" />\n            </intent-filter>\n            <intent-filter>\n                <action android:name=\"android.intent.action.DIAL\" />\n                <category android:name=\"android.intent.category.DEFAULT\" />\n                <data android:scheme=\"tel\" />\n            </intent-filter>\n");
    }
    // O som dos outros apps (`loopback:Android` no Microphone ou no AudioMixer): o serviço da
    // captura, que o Android 10+ exige rodando enquanto ela dura.
    if uses_android_loopback(&forms) {
        provider.push_str("        <service android:name=\"dev.aylis.AylisActivity$Projection\" android:exported=\"false\" android:foregroundServiceType=\"mediaProjection\" />
");
    }
    // O BackgroundService: o serviço em primeiro plano do tipo dele e, com keep_alive, o boot.
    if let Some((kind, keep_alive)) = background_service(&forms) {
        provider.push_str(&format!("        <service android:name=\"dev.aylis.AylisActivity$Background\" android:exported=\"false\" android:foregroundServiceType=\"{}\" />\n", xml_escape(&kind)));
        if keep_alive {
            provider.push_str("        <receiver android:name=\"dev.aylis.AylisActivity$Boot\" android:exported=\"true\">\n            <intent-filter>\n                <action android:name=\"android.intent.action.BOOT_COMPLETED\" />\n            </intent-filter>\n        </receiver>\n");
        }
    }
    // O push do FCM: o receiver das mensagens (só o Play Services manda, pela permissão
    // SEND) e o Play Services visível ao app (Android 11+, para pedir o token a ele).
    if aylis_build::used_classes(&forms).iter().any(|c| c == "PushNotifications") {
        provider.push_str("        <receiver android:name=\"dev.aylis.AylisActivity$Push\" android:exported=\"true\" android:permission=\"com.google.android.c2dm.permission.SEND\">\n            <intent-filter>\n                <action android:name=\"com.google.android.c2dm.intent.RECEIVE\" />\n            </intent-filter>\n        </receiver>\n");
        permissions.push_str("    <queries>\n        <package android:name=\"com.google.android.gms\" />\n    </queries>\n");
    }
    // As ações do aparelho: o receiver dos botões que respondem sem abrir o app e do fim do
    // tempo (só o próprio app manda).
    if aylis_build::used_classes(&forms).iter().any(|c| c == "WakeAppAction" || c == "NotifyAction") {
        provider.push_str("        <receiver android:name=\"dev.aylis.AylisActivity$Actions\" android:exported=\"false\" />\n");
    }
    // Um link ou um compartilhar chega ao app já aberto (`onNewIntent`), e não numa segunda
    // activity na tarefa do outro app.
    let single = if receives_shares || !dialer.is_empty() || a.deep_links.iter().any(|l| !l.trim().is_empty()) { "\n            android:launchMode=\"singleTask\"" } else { "" };
    let mut links = String::new();
    links.push_str(&dialer);
    if receives_shares {
        links.push_str("            <intent-filter>\n                <action android:name=\"android.intent.action.SEND\" />\n                <action android:name=\"android.intent.action.SEND_MULTIPLE\" />\n                <category android:name=\"android.intent.category.DEFAULT\" />\n                <data android:mimeType=\"*/*\" />\n            </intent-filter>\n");
    }
    for link in a.deep_links.iter().filter(|l| !l.trim().is_empty()) {
        let (scheme, rest) = link.split_once("://").unwrap_or((link.as_str(), ""));
        let (host, path) = rest.split_once('/').map_or((rest, String::new()), |(h, p)| (h, format!("/{p}")));
        let mut data = format!("android:scheme=\"{}\"", xml_escape(scheme));
        if !host.is_empty() {
            data.push_str(&format!(" android:host=\"{}\"", xml_escape(host)));
        }
        if !path.is_empty() && path != "/" {
            data.push_str(&format!(" android:pathPrefix=\"{}\"", xml_escape(&path)));
        }
        let verify = if scheme == "https" { " android:autoVerify=\"true\"" } else { "" };
        links.push_str(&format!(
            "            <intent-filter{verify}>\n                <action android:name=\"android.intent.action.VIEW\" />\n                <category android:name=\"android.intent.category.DEFAULT\" />\n                <category android:name=\"android.intent.category.BROWSABLE\" />\n                <data {data} />\n            </intent-filter>\n"
        ));
    }
    let base = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<!-- Gerado pelo aylis. A AylisActivity (uma NativeActivity com o teclado completo) carrega lib{package}.so e chama android_main. -->
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="{id}"
    android:versionCode="{version_code}"
    android:versionName="{version}">

    <uses-sdk android:minSdkVersion="{min_sdk}" android:targetSdkVersion="{target_sdk}" />
{permissions}
    <application
        android:label="{label}"{icon}
        android:hasCode="true"
        android:theme="{theme}">
        <activity
            android:name="dev.aylis.AylisActivity"
            android:exported="true"{orientation}{keep_on}{single}
            android:configChanges="orientation|screenSize|screenLayout|smallestScreenSize|keyboard|keyboardHidden|navigation|uiMode|density"
            android:windowSoftInputMode="adjustResize|stateAlwaysHidden">
            <meta-data android:name="android.app.lib_name" android:value="{package}" />
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
{links}        </activity>
{provider}    </application>
</manifest>
"#,
        id = project.identifier(),
        version = project.version(),
        label = xml_escape(project.app_name()),
    );
    match std::fs::read_to_string(project.file(ANDROID_MANIFEST_EXTRA)) {
        Ok(extra) => merge_manifest_extra(&base, &extra),
        Err(_) => Ok(base),
    }
}

/// Mescla o XML a mais (`android/manifest-extra.xml`): um `<manifest>` cujos filhos entram no
/// gerado; os de dentro de um `<application>` entram na aplicação. Confere se é XML válido.
pub fn merge_manifest_extra(base: &str, extra: &str) -> Result<String, String> {
    let doc = roxmltree::Document::parse(extra).map_err(|e| format!("{ANDROID_MANIFEST_EXTRA}: {e}"))?;
    let root = doc.root_element();
    if root.tag_name().name() != "manifest" {
        return Err(format!("{ANDROID_MANIFEST_EXTRA}: a raiz deve ser <manifest>"));
    }
    let text = |node: roxmltree::Node| extra[node.range()].to_string();
    let (mut top, mut app) = (String::new(), String::new());
    for child in root.children().filter(|c| c.is_element()) {
        if child.tag_name().name() == "application" {
            for inner in child.children().filter(|c| c.is_element()) {
                app.push_str(&format!("        {}\n", text(inner)));
            }
        } else {
            top.push_str(&format!("    {}\n", text(child)));
        }
    }
    let mut out = base.to_string();
    if !app.is_empty() {
        let at = out.rfind("    </application>").ok_or("manifesto sem </application>")?;
        out.insert_str(at, &format!("        <!-- de {ANDROID_MANIFEST_EXTRA} -->\n{app}"));
    }
    if !top.is_empty() {
        let at = out.find("    <application").ok_or("manifesto sem <application>")?;
        out.insert_str(at, &format!("    <!-- de {ANDROID_MANIFEST_EXTRA} -->\n{top}"));
    }
    roxmltree::Document::parse(&out).map_err(|e| format!("o manifesto mesclado não é XML válido: {e}"))?;
    Ok(out)
}

/// O ícone do projeto em `res/`: os 5 tamanhos e o adaptativo (a cor de fundo do
/// `icon_background`, ou a da borda da imagem). `false` se não tiver.
fn android_icon(project: &Project, work: &Path) -> Result<bool, String> {
    if project.publish.icon.is_empty() {
        return Ok(false);
    }
    let icon = crate::icons::Icon::open(&project.file(&project.publish.icon))?;
    let background = Color::from_hex(&project.publish.icon_background).map(|c| [c.r, c.g, c.b]);
    crate::icons::android(&icon, &work.join("res"), background)?;
    Ok(true)
}

/// O AAB (Android App Bundle, o formato da Play Store): os recursos em protobuf
/// (`aapt2 --proto-format`), o módulo `base` montado à mão, o `bundletool` e a assinatura
/// com o `jarsigner`.
#[allow(clippy::too_many_arguments)]
fn android_bundle(
    project: &Project,
    tools: &AndroidTools,
    work: &Path,
    out: &Path,
    has_icon: bool,
    manifest: &Path,
    (keystore, alias, store_pass, key_pass): (&Path, &str, &str, &str),
    developer_key: bool,
    control: &Control,
    sink: Sink,
) -> Result<Published, String> {
    let bundletool = bundletool().ok_or("bundletool não encontrado (Ferramentas → Verificar ambiente baixa)")?;
    sink(Event::Step("Montando o AAB…".into()));
    let aapt2 = tools.build_tools.join(exe("aapt2"));
    let proto = work.join("proto.apk");
    let mut link = Command::new(&aapt2);
    link.arg("link").arg("--proto-format").arg("-o").arg(&proto).arg("--manifest").arg(manifest).arg("-I").arg(&tools.platform_jar);
    if has_icon {
        link.arg("-R").arg(work.join("res.zip")).arg("--auto-add-overlay");
    }
    run_tool(&mut link, control, sink, LineKind::Tool)?;
    // O módulo: manifest/, dex/, lib/, res/ e resources.pb.
    let module = work.join("base");
    std::fs::create_dir_all(&module).map_err(|e| e.to_string())?;
    let jar = tools.java.join(exe("jar"));
    run_tool(Command::new(&jar).arg("--extract").arg("--file").arg(&proto).current_dir(&module), control, sink, LineKind::Tool)?;
    std::fs::create_dir_all(module.join("manifest")).map_err(|e| e.to_string())?;
    std::fs::rename(module.join("AndroidManifest.xml"), module.join("manifest/AndroidManifest.xml")).map_err(|e| format!("manifesto: {e}"))?;
    std::fs::create_dir_all(module.join("dex")).map_err(|e| e.to_string())?;
    std::fs::copy(work.join("dex/classes.dex"), module.join("dex/classes.dex")).map_err(|e| format!("classes.dex: {e}"))?;
    copy_dir(&work.join("lib"), &module.join("lib"))?;
    let base_zip = work.join("base.zip");
    run_tool(Command::new(&jar).arg("--create").arg("--no-manifest").arg("--file").arg(&base_zip).arg("-C").arg(&module).arg("."), control, sink, LineKind::Tool)?;
    let aab = out.join(format!("{}-{}.aab", file_name(project.app_name()), project.version()));
    let _ = std::fs::remove_file(&aab);
    run_tool(
        Command::new(tools.java.join(exe("java")))
            .arg("-jar")
            .arg(&bundletool)
            .arg("build-bundle")
            .arg(format!("--modules={}", base_zip.display()))
            .arg(format!("--output={}", aab.display())),
        control,
        sink,
        LineKind::Tool,
    )?;
    run_tool(
        Command::new(tools.java.join(exe("jarsigner")))
            .args(["-sigalg", "SHA256withRSA", "-digestalg", "SHA-256", "-keystore"])
            .arg(keystore)
            .args(["-storepass:env", "AYLIS_SIGN_STORE", "-keypass:env", "AYLIS_SIGN_KEY"])
            .arg(&aab)
            .arg(alias)
            .env("AYLIS_SIGN_STORE", store_pass)
            .env("AYLIS_SIGN_KEY", key_pass),
        control,
        sink,
        LineKind::Tool,
    )?;
    let _ = std::fs::remove_dir_all(work);
    sink(Event::Line(format!("{} ({})", aab.display(), size(&aab)), LineKind::Tool));
    if !developer_key {
        sink(Event::Line("Assinado com a chave de depuração: a Play Store recusa; configure a chave do desenvolvedor (Projeto → Opções).".into(), LineKind::Tool));
    }
    Ok(Published { main: aab, dir: out.to_path_buf() })
}

/// A chave de depuração do Android (criada na primeira vez, como o Android Studio faz).
fn debug_keystore(tools: &AndroidTools, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    let home = std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from).ok_or("pasta do usuário")?;
    let keystore = home.join(".android/debug.keystore");
    if !keystore.is_file() {
        std::fs::create_dir_all(keystore.parent().expect("tem pai")).map_err(|e| e.to_string())?;
        run_tool(
            Command::new(tools.java.join(exe("keytool")))
                .args(["-genkeypair", "-keystore"])
                .arg(&keystore)
                .args(["-storepass", "android", "-alias", "androiddebugkey", "-keypass", "android", "-keyalg", "RSA", "-keysize", "2048", "-validity", "10000"])
                .args(["-dname", "CN=Android Debug,O=Android,C=US"]),
            control,
            sink,
            LineKind::Tool,
        )?;
    }
    Ok(keystore)
}

fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

// ---------------------------------------------------------------------------
// Web

/// O `Cargo.lock` que vale para o crate em `dir`: o dele ou o do workspace acima.
fn cargo_lock_of(dir: &Path) -> Option<PathBuf> {
    dir.ancestors().map(|d| d.join("Cargo.lock")).find(|p| p.is_file())
}

/// O `wasm-bindgen` da mesma versão que o projeto usou: o formato do `.wasm` muda a cada
/// versão, e um projeto novo resolve a mais recente. A do PATH, se for igual; senão uma
/// instalada pelo aylis (uma vez por versão) na pasta de ferramentas dele.
fn wasm_bindgen_tool(crate_dir: &Path, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    let wanted = cargo_lock_of(crate_dir).and_then(|lock| env::lock_version(&lock, "wasm-bindgen"));
    let Some(wanted) = wanted else { return Ok(PathBuf::from("wasm-bindgen")) };
    let installed = Command::new("wasm-bindgen")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|v| v.split_whitespace().nth(1).map(str::to_string));
    if installed.as_deref() == Some(wanted.as_str()) {
        return Ok(PathBuf::from("wasm-bindgen"));
    }
    let root = crate::dap::tools_dir().ok_or("sem pasta de ferramentas (LOCALAPPDATA/HOME)")?.join(format!("wasm-bindgen-{wanted}"));
    let tool = root.join("bin").join(exe("wasm-bindgen"));
    if !tool.is_file() {
        sink(Event::Step(format!("Instalando o wasm-bindgen {wanted}, a versão que o projeto usa (uma vez só)…")));
        run_tool(
            Command::new("cargo").args(["install", "wasm-bindgen-cli", "--version", &wanted, "--locked", "--root"]).arg(&root),
            control,
            sink,
            LineKind::Tool,
        )?;
    }
    if tool.is_file() { Ok(tool) } else { Err(format!("o wasm-bindgen {wanted} não ficou em {}", tool.display())) }
}

/// O perfil de release das compilações para o navegador: o wasm é baixado (no celular, pela
/// rede móvel) e compilado pelo navegador a cada visita nova, então o tamanho pesa mais que
/// alguns por cento de velocidade. LTO com uma unidade só tira o código que nenhum form usa,
/// `opt-level = "s"` otimiza para tamanho e `strip` tira os nomes das funções (a seção `name`,
/// só para depuração: 1,4 MB em cada wasm; a mensagem de um pânico continua com o arquivo e a
/// linha). Medido na família Standard das demonstrações: 6,6 → 3,7 MB (1,91 → 1,35 MB
/// comprimido); veja `medicoes/web_wasm.json`. Pelas variáveis do cargo, valendo também para
/// os projetos de fora (o `Cargo.toml` deles não muda) e sem mexer no release do desktop.
const WEB_RELEASE: [(&str, &str); 4] = [
    ("CARGO_PROFILE_RELEASE_LTO", "fat"),
    ("CARGO_PROFILE_RELEASE_CODEGEN_UNITS", "1"),
    ("CARGO_PROFILE_RELEASE_OPT_LEVEL", "s"),
    ("CARGO_PROFILE_RELEASE_STRIP", "symbols"),
];

/// [`WEB_RELEASE`] no formato do [`cargo`].
fn web_release_env<'a>() -> Vec<(&'a str, &'a Path)> {
    WEB_RELEASE.iter().map(|(k, v)| (*k, Path::new(*v))).collect()
}

fn web(project: &Project, options: &Options, out: &Path, control: &Control, sink: Sink) -> Result<Published, String> {
    sink(Event::Step("Compilando para WebAssembly…".into()));
    let package = project.package_name();
    let mut args = vec!["rustc", "--lib", "--crate-type", "cdylib", "--target", "wasm32-unknown-unknown"];
    if !options.debug {
        args.push("--release");
    }
    let features = options.features.join(",");
    if !features.is_empty() {
        args.extend(["--features", features.as_str()]);
    }
    let crate_dir = options.crate_dir.clone().unwrap_or_else(|| project.code_dir());
    let envs = if options.debug { Vec::new() } else { web_release_env() };
    let built = cargo("cargo", &args, &crate_dir, &envs, control, sink)?;
    let wasm = built
        .artifacts
        .iter()
        .flat_map(|a| a.filenames.iter())
        .find(|f| f.extension().is_some_and(|e| e == "wasm"))
        .cloned()
        .unwrap_or_else(|| crate::shared_target_dir().join("wasm32-unknown-unknown").join(profile(options)).join(format!("{package}.wasm")));
    let bindgen = wasm_bindgen_tool(&crate_dir, control, sink)?;
    sink(Event::Step("Gerando o pacote web (wasm-bindgen)…".into()));
    run_tool(
        Command::new(&bindgen).args(["--target", "web", "--no-typescript", "--out-dir"]).arg(out.join("pkg")).arg(&wasm),
        control,
        sink,
        LineKind::Tool,
    )?;
    // O ícone da aba (o `.ico` com 16/32/48 e o PNG) e o da tela inicial do celular.
    let mut favicon = String::new();
    if !project.publish.icon.is_empty() {
        let icon = crate::icons::Icon::open(&project.file(&project.publish.icon))?;
        crate::icons::web(&icon, out)?;
        favicon = WEB_ICON_LINKS.into();
    }
    let (js, wasm_file) = cache_web_page(out, &package)?;
    let preload = web_preload(&js, &wasm_file);
    let index = out.join("index.html");
    let html = format!(
        r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title}</title>{favicon}{preload}
  <style>
    html, body {{ margin: 0; height: 100%; overflow: hidden; background: #f3f4f6; }}
    @media (prefers-color-scheme: dark) {{ html, body {{ background: #111; }} }}
    /* O form ocupa a janela do navegador (como no celular). */
    canvas {{ display: block; outline: none; width: 100vw !important; height: 100vh !important; }}
  </style>
</head>
<body>
  <!-- Gerado pelo Aylis. Sirva esta pasta com qualquer servidor web (o .wasm não abre direto do disco). -->
  <script type="module">
    import init from "./pkg/{js}";
    const wasm = await init({{ module_or_path: "./pkg/{wasm_file}" }});
    wasm.aylis_start();
    {sw}
  </script>
</body>
</html>
"#,
        title = xml_escape(project.app_name()),
        sw = REGISTER_SERVICE_WORKER,
    );
    std::fs::write(&index, html).map_err(|e| e.to_string())?;
    let wasm_out = out.join("pkg").join(&wasm_file);
    sink(Event::Line(format!("{} ({} de wasm)", out.display(), size(&wasm_out)), LineKind::Tool));
    Ok(Published { main: index, dir: out.to_path_buf() })
}

/// A IDE lite no navegador (o "Experimentar" da página do aylis): compila o `aylis-ide-web`
/// para WebAssembly e grava em `out` a página que a carrega (`index.html` e `pkg/`).
pub fn ide_web(out: &Path, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    let root = crate::aylis_root();
    let crate_dir = root.join("crates").join("aylis-ide-web");
    sink(Event::Step("Compilando a IDE lite para WebAssembly…".into()));
    let args = ["rustc", "--lib", "--crate-type", "cdylib", "--target", "wasm32-unknown-unknown", "--release"];
    let built = cargo("cargo", &args, &crate_dir, &web_release_env(), control, sink)?;
    let wasm = built
        .artifacts
        .iter()
        .flat_map(|a| a.filenames.iter())
        .find(|f| f.extension().is_some_and(|e| e == "wasm"))
        .cloned()
        .ok_or("o cargo não entregou o .wasm da IDE")?;
    let bindgen = wasm_bindgen_tool(&crate_dir, control, sink)?;
    sink(Event::Step("Gerando o pacote web (wasm-bindgen)…".into()));
    run_tool(Command::new(&bindgen).args(["--target", "web", "--no-typescript", "--out-dir"]).arg(out.join("pkg")).arg(&wasm), control, sink, LineKind::Tool)?;
    let (js, wasm_file) = cache_web_page(out, "aylis_ide_web")?;
    let preload = web_preload(&js, &wasm_file);
    // O ícone da aba: o da IDE de desktop.
    let favicon = match std::fs::copy(root.join("crates").join("aylis-ide").join("resources").join("aylis-ide.ico"), out.join("favicon.ico")) {
        Ok(_) => "\n  <link rel=\"icon\" href=\"favicon.ico\" sizes=\"any\">",
        Err(_) => "",
    };
    let index = out.join("index.html");
    let html = format!(
        r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Aylis — a IDE no navegador</title>{favicon}{preload}
  <style>
    html, body {{ margin: 0; height: 100%; overflow: hidden; background: #1e1f22; color: #c9ccd3; font: 15px system-ui, sans-serif; }}
    canvas {{ display: block; outline: none; width: 100vw !important; height: 100vh !important; }}
    #carregando {{ position: fixed; inset: 0; display: grid; place-items: center; }}
  </style>
</head>
<body>
  <!-- Gerado pelo aylis-tool ide-web. Sirva esta pasta com qualquer servidor web. -->
  <div id="carregando">Carregando a IDE do Aylis…</div>
  <script type="module">
    import init from "./pkg/{js}";
    const wasm = await init({{ module_or_path: "./pkg/{wasm_file}" }});
    document.getElementById("carregando").remove();
    wasm.aylis_ide_start();
    {sw}
  </script>
</body>
</html>
"#,
        sw = REGISTER_SERVICE_WORKER,
    );
    std::fs::write(&index, html).map_err(|e| e.to_string())?;
    let wasm_out = out.join("pkg").join(&wasm_file);
    sink(Event::Line(format!("{} ({} de wasm)", out.display(), size(&wasm_out)), LineKind::Tool));
    Ok(index)
}

/// Uma família das demonstrações: a categoria da paleta, a pasta dela (`demos/<pasta>/`) e as
/// features do `aylis-demos` que as classes dela pedem.
pub struct DemoFamily {
    pub category: String,
    pub folder: String,
    pub features: Vec<&'static str>,
    pub classes: Vec<String>,
}

/// As famílias das demonstrações (as categorias da paleta com alguma classe que roda no
/// navegador), na ordem em que aparecem no `aylis::classes`.
pub fn demo_families() -> Vec<DemoFamily> {
    let facade = std::fs::read_to_string(crate::aylis_root().join("crates").join("aylis").join("src").join("lib.rs")).unwrap_or_default();
    let mut families: Vec<DemoFamily> = Vec::new();
    for (class, category) in aylis_build::class_categories(&facade) {
        if !aylis_build::demo_on_web(class) {
            continue;
        }
        let at = match families.iter().position(|f| f.category == category) {
            Some(i) => i,
            None => {
                families.push(DemoFamily { category: category.to_string(), folder: demo_folder(category), features: Vec::new(), classes: Vec::new() });
                families.len() - 1
            }
        };
        let family = &mut families[at];
        family.classes.push(class.to_string());
        if let Some(f) = aylis_build::demo_feature(class).filter(|f| !family.features.contains(f)) {
            family.features.push(f);
        }
    }
    families
}

/// A pasta de uma família: a categoria em minúsculas, com `-` no lugar dos espaços
/// ("Data Controls" → `data-controls`). A documentação monta o mesmo caminho.
pub fn demo_folder(category: &str) -> String {
    category.trim().to_lowercase().replace(' ', "-")
}

/// Os componentes de verdade da documentação: um wasm por família ([`demo_families`]), cada
/// um em `out/<pasta>/` com a página que o carrega (`index.html?class=Button&theme=dark`).
/// `only`: só as famílias com esses nomes (a categoria ou a pasta).
pub fn demos(out: &Path, only: &[String], control: &Control, sink: Sink) -> Result<Vec<PathBuf>, String> {
    let crate_dir = crate::aylis_root().join("crates").join("aylis-demos");
    let bindgen = wasm_bindgen_tool(&crate_dir, control, sink)?;
    let mut pages = Vec::new();
    let families = demo_families();
    for family in families.iter().filter(|f| only.is_empty() || only.iter().any(|o| o.eq_ignore_ascii_case(&f.category) || *o == f.folder)) {
        sink(Event::Step(format!("Compilando a família {} ({} componentes)…", family.category, family.classes.len())));
        let features = family.features.join(",");
        let mut args = vec!["rustc", "--lib", "--crate-type", "cdylib", "--target", "wasm32-unknown-unknown", "--release"];
        if !features.is_empty() {
            args.extend(["--features", features.as_str()]);
        }
        let mut envs = web_release_env();
        envs.push(("AYLIS_DEMO_FAMILY", Path::new(&family.category)));
        let built = cargo("cargo", &args, &crate_dir, &envs, control, sink)?;
        let wasm = built
            .artifacts
            .iter()
            .flat_map(|a| a.filenames.iter())
            .find(|f| f.extension().is_some_and(|e| e == "wasm"))
            .cloned()
            .ok_or("o cargo não entregou o .wasm das demonstrações")?;
        let dir = out.join(&family.folder);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        run_tool(Command::new(&bindgen).args(["--target", "web", "--no-typescript", "--out-dir"]).arg(dir.join("pkg")).arg(&wasm), control, sink, LineKind::Tool)?;
        let (js, wasm_file) = cache_web_page(&dir, "aylis_demos")?;
        let preload = web_preload(&js, &wasm_file);
        let html = format!(
            r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Aylis — {category}</title>{preload}
  <style>
    html, body {{ margin: 0; height: 100%; overflow: hidden; background: var(--bg, #f3f4f6); color: #555; font: 14px system-ui, sans-serif; }}
    html.dark, html.dark body {{ background: var(--bg, #1e1f24); color: #9aa0ad; }}
    canvas {{ display: block; outline: none; width: 100vw !important; height: 100vh !important; }}
    /* O skeleton até o componente desenhar: o mesmo do `WebView` com `skeleton` (a barra e o
       bloco, nas mesmas medidas e no mesmo tom), para a troca do canvas para o `<iframe>` não
       aparecer; aqui com um brilho passando (só CSS). */
    #carregando {{ position: fixed; inset: 0; z-index: 1; box-sizing: border-box; padding: clamp(8px, 8vmin, 24px); display: flex; flex-direction: column; gap: 14px; --osso: rgba(17,24,39,.07); --luz: rgba(17,24,39,.13); }}
    html.dark #carregando {{ --osso: rgba(230,232,239,.07); --luz: rgba(230,232,239,.13); }}
    #carregando b, #carregando i {{ display: block; background: linear-gradient(100deg, var(--osso) 30%, var(--luz) 50%, var(--osso) 70%); background-size: 200% 100%; animation: brilho 1.4s linear infinite; }}
    #carregando b {{ width: 45%; height: 14px; border-radius: 7px; }}
    #carregando i {{ flex: 1; border-radius: 10px; }}
    @keyframes brilho {{ from {{ background-position: 100% 0; }} to {{ background-position: -100% 0; }} }}
    @media (prefers-reduced-motion: reduce) {{ #carregando b, #carregando i {{ animation: none; }} }}
  </style>
  <script>
    // O tema e o fundo da página em volta (`bg`), antes do wasm chegar; com `fill=1` (o
    // componente como parte da página, o lingote do topo), o fundo da página.
    const q = new URLSearchParams(location.search);
    if (q.get("theme") !== "light") document.documentElement.classList.add("dark");
    const bg = q.get("bg");
    if (/^[0-9a-fA-F]{{6}}$/.test(bg || "")) document.documentElement.style.setProperty("--bg", '#' + bg);
    if (q.get("fill") === "1") document.documentElement.classList.add("fill");
  </script>
</head>
<body>
  <!-- Gerado pelo aylis-tool demos: a família {category} de componentes de verdade. -->
  <div id="carregando" role="progressbar" aria-label="Carregando"><b></b><i></i></div>
  <script type="module">
    import init from "./pkg/{js}";
    const wasm = await init({{ module_or_path: "./pkg/{wasm_file}" }});
    wasm.aylis_demo_start();
    // Sai depois do primeiro quadro do componente (antes, ficaria um instante só o fundo).
    requestAnimationFrame(() => requestAnimationFrame(() => document.getElementById("carregando").remove()));
    {sw}
  </script>
</body>
</html>
"#,
            category = xml_escape(&family.category),
            sw = REGISTER_SERVICE_WORKER,
        );
        let index = dir.join("index.html");
        std::fs::write(&index, html).map_err(|e| e.to_string())?;
        sink(Event::Line(format!("{} ({} de wasm)", dir.display(), size(&dir.join("pkg").join(&wasm_file))), LineKind::Tool));
        pages.push(index);
    }
    Ok(pages)
}

/// O service worker das páginas web: o `.js` e o `.wasm` (com o hash do conteúdo no nome)
/// saem do cache do navegador, sem perguntar ao servidor; o resto (o `index.html`, pequeno)
/// pergunta primeiro e cai no cache sem rede. Uma publicação nova muda o `index.html`, que
/// aponta para os nomes novos: só eles são baixados. Cada pasta (a página, a `ide/`) tem o
/// seu cache.
///
/// A página, o `.js` e o `.wasm` entram no cache já na instalação (`@FILES@`): a primeira
/// visita carrega tudo antes de o service worker existir, e sem isso a página nunca ia para o
/// cache (sem servidor, o F5 dava `ERR_FAILED`). O cache antigo só sai depois de o novo estar
/// completo, e a navegação acha a página guardada com query (`?class=Button`), com o
/// `index.html` no fim ou sem a barra final.
const SERVICE_WORKER: &str = r#"// Gerado pelo Aylis: guarda a página no navegador para abrir rápido (e sem rede).
const SCOPE = self.registration.scope;
const PREFIX = "aylis:" + SCOPE + ":";
const CACHE = PREFIX + "@VERSION@";
// A página (a pasta, com a barra no fim) e o que ela carrega antes de desenhar.
const PRECACHE = ["./", @FILES@];
// Com o hash no nome: o conteúdo nunca muda.
const IMMUTABLE = /-[0-9a-f]{10}\.(js|wasm)$/;

// A resposta sem a marca de redirecionamento: o navegador recusa uma resposta redirecionada
// para uma navegação (o ERR_FAILED).
async function clean(response) {
  if (!response.redirected) return response;
  return new Response(await response.blob(), { status: response.status, statusText: response.statusText, headers: response.headers });
}

// A chave de uma navegação: sem a query e, na página da pasta, sem o `index.html`.
function pageKey(url) {
  const key = new URL(url.href);
  key.search = "";
  key.hash = "";
  if (key.pathname.endsWith("/index.html")) key.pathname = key.pathname.slice(0, -"index.html".length);
  return key.href;
}

self.addEventListener("install", (event) => {
  // Sem rede (ou com um arquivo faltando) a instalação falha e o service worker anterior,
  // com o cache dele, continua valendo.
  event.waitUntil((async () => {
    const cache = await caches.open(CACHE);
    for (const file of PRECACHE) {
      const url = new URL(file, SCOPE).href;
      const response = await fetch(new Request(url, { cache: "reload" }));
      if (!response.ok) throw new Error(url + ": " + response.status);
      await cache.put(url, await clean(response));
    }
    await self.skipWaiting();
  })());
});

self.addEventListener("activate", (event) => {
  event.waitUntil((async () => {
    const cache = await caches.open(CACHE);
    for (const key of await caches.keys()) {
      if (!key.startsWith(PREFIX) || key === CACHE) continue;
      // O que a página baixou depois (fontes, imagens, outras páginas) passa para o cache
      // novo, para continuar abrindo sem rede; o `.js` e o `.wasm` antigos não servem mais.
      const old = await caches.open(key);
      for (const request of await old.keys()) {
        if (IMMUTABLE.test(new URL(request.url).pathname) || (await cache.match(request))) continue;
        const response = await old.match(request);
        if (response) await cache.put(request, response);
      }
      await caches.delete(key);
    }
    await self.clients.claim();
  })());
});

self.addEventListener("fetch", (event) => {
  const request = event.request;
  const url = new URL(request.url);
  if (request.method !== "GET" || url.origin !== location.origin) return;
  const navigate = request.mode === "navigate";
  const key = navigate ? pageKey(url) : request;
  event.respondWith((async () => {
    const cache = await caches.open(CACHE);
    if (IMMUTABLE.test(url.pathname)) {
      const hit = await cache.match(request);
      if (hit) return hit;
    }
    try {
      const response = await fetch(request);
      if (response.ok && response.type === "basic") {
        const copy = await clean(response.clone());
        event.waitUntil(cache.put(key, copy));
      }
      return response;
    } catch (error) {
      // Sem rede: a cópia guardada (deste cache ou, na troca de versão, de outro).
      const hit = (await cache.match(key)) || (await caches.match(key)) || (await caches.match(request, { ignoreSearch: true }));
      if (hit) return hit;
      // `/ide` sem a barra: o servidor redirecionaria para `/ide/`; sem ele, quem redireciona
      // é aqui, se a pasta estiver guardada.
      if (navigate && !url.pathname.endsWith("/") && !url.pathname.split("/").pop().includes(".")) {
        const folder = new URL(url.href);
        folder.pathname += "/";
        if (await caches.match(pageKey(folder))) return Response.redirect(folder.href, 302);
      }
      throw error;
    }
  })());
});

// Web Push (o componente PushNotifications): a mensagem vira uma linha para a página
// ("message\tid\ttítulo\ttexto\tdados JSON"). Com a página na frente, quem decide mostrar é
// ela; senão, a notificação aparece aqui (o navegador exige).
const field = (text) => String(text ?? "").replace(/[\t\r\n]/g, " ");
function pushLine(payload) {
  let p = {};
  try { p = JSON.parse(payload); } catch (_) { p = { body: payload }; }
  if (typeof p !== "object" || p === null) p = { body: String(payload) };
  const n = p.notification || {};
  const data = p.data && typeof p.data === "object" ? p.data : {};
  return [field(p.id), field(p.title ?? n.title), field(p.body ?? n.body), JSON.stringify(data)].join("\t");
}

self.addEventListener("push", (event) => {
  const line = pushLine(event.data ? event.data.text() : "");
  event.waitUntil((async () => {
    const windows = await self.clients.matchAll({ type: "window", includeUncontrolled: true });
    for (const w of windows) w.postMessage({ aylis_push: "message\t" + line });
    if (windows.some((w) => w.focused || w.visibilityState === "visible")) return;
    const [, title, body] = line.split("\t");
    await self.registration.showNotification(title || self.registration.scope, { body, data: { aylis_push: line } });
  })());
});

self.addEventListener("notificationclick", (event) => {
  event.notification.close();
  const line = (event.notification.data && event.notification.data.aylis_push) || "";
  event.waitUntil((async () => {
    const windows = await self.clients.matchAll({ type: "window", includeUncontrolled: true });
    if (windows.length) {
      const w = windows[0];
      w.postMessage({ aylis_push: "opened\t" + line });
      if ("focus" in w) await w.focus();
    } else {
      await self.clients.openWindow(self.registration.scope + "?aylis_push=" + encodeURIComponent(line));
    }
  })());
});
"#;

/// Deixa a página web rápida de abrir de novo: põe o hash do conteúdo no nome do `.js` e do
/// `.wasm` (só mudam de nome quando mudam), apaga os de publicações anteriores e grava o
/// `sw.js` ([`SERVICE_WORKER`]). Devolve os nomes novos (em `pkg/`).
fn cache_web_page(out: &Path, package: &str) -> Result<(String, String), String> {
    use sha2::{Digest, Sha256};
    let pkg = out.join("pkg");
    let (js, wasm) = (pkg.join(format!("{package}.js")), pkg.join(format!("{package}_bg.wasm")));
    let mut hasher = Sha256::new();
    hasher.update(std::fs::read(&wasm).map_err(|e| format!("{}: {e}", wasm.display()))?);
    hasher.update(std::fs::read(&js).map_err(|e| format!("{}: {e}", js.display()))?);
    let hash: String = hasher.finalize().iter().take(5).map(|b| format!("{b:02x}")).collect();
    let (js_name, wasm_name) = (format!("{package}-{hash}.js"), format!("{package}_bg-{hash}.wasm"));
    if let Ok(entries) = std::fs::read_dir(&pkg) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let old = (name.starts_with(&format!("{package}-")) && name.ends_with(".js")) || (name.starts_with(&format!("{package}_bg-")) && name.ends_with(".wasm"));
            if old {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
    std::fs::rename(&js, pkg.join(&js_name)).map_err(|e| e.to_string())?;
    std::fs::rename(&wasm, pkg.join(&wasm_name)).map_err(|e| e.to_string())?;
    std::fs::write(out.join("sw.js"), service_worker(&hash, &js_name, &wasm_name)).map_err(|e| e.to_string())?;
    Ok((js_name, wasm_name))
}

/// O texto do `sw.js` desta publicação: a versão do cache e o que entra nele na instalação.
fn service_worker(hash: &str, js: &str, wasm: &str) -> String {
    SERVICE_WORKER.replace("@VERSION@", hash).replace("@FILES@", &format!("\"./pkg/{js}\", \"./pkg/{wasm}\""))
}

/// O ícone da aba do navegador e o da tela inicial do celular ([`crate::icons::web`]).
const WEB_ICON_LINKS: &str = "\n  <link rel=\"icon\" href=\"favicon.ico\" sizes=\"any\">\n  <link rel=\"icon\" type=\"image/png\" href=\"icon.png\">\n  <link rel=\"apple-touch-icon\" href=\"icon-192.png\">";

/// As linhas do `<head>` que começam a baixar o `.js` e o `.wasm` junto com a página.
fn web_preload(js: &str, wasm: &str) -> String {
    format!("\n  <link rel=\"modulepreload\" href=\"./pkg/{js}\">\n  <link rel=\"preload\" href=\"./pkg/{wasm}\" as=\"fetch\" type=\"application/wasm\" crossorigin>")
}

/// Liga o service worker (só em https ou no localhost; fora deles, o navegador ignora).
const REGISTER_SERVICE_WORKER: &str = r#"if ("serviceWorker" in navigator) navigator.serviceWorker.register("sw.js").catch(() => {});"#;

// ---------------------------------------------------------------------------
// iOS

fn ios(project: &Project, options: &Options, out: &Path, control: &Control, sink: Sink) -> Result<Published, String> {
    if !cfg!(target_os = "macos") {
        return Err("o app de iOS só é gerado num Mac (o Xcode é obrigatório)".into());
    }
    let triple = if std::env::consts::ARCH == "aarch64" { "aarch64-apple-ios-sim" } else { "x86_64-apple-ios" };
    sink(Event::Step(format!("Compilando para iOS ({triple})…")));
    let package = project.bin_name();
    let mut args = vec!["build", "--bin", package.as_str(), "--target", triple];
    if !options.debug {
        args.push("--release");
    }
    let built = cargo("cargo", &args, &project.code_dir(), &[], control, sink)?;
    let bin = built.artifacts.iter().find_map(|a| a.executable.clone()).ok_or("o cargo não informou o executável")?;
    let app = out.join(format!("{}.app", file_name(project.app_name())));
    std::fs::create_dir_all(&app).map_err(|e| e.to_string())?;
    std::fs::copy(&bin, app.join(&package)).map_err(|e| e.to_string())?;
    let push = aylis_build::used_classes(&project.dir().join("forms")).iter().any(|c| c == "PushNotifications");
    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key><string>{package}</string>
  <key>CFBundleIdentifier</key><string>{id}</string>
  <key>CFBundleName</key><string>{name}</string>
  <key>CFBundleDisplayName</key><string>{name}</string>
  <key>CFBundleShortVersionString</key><string>{version}</string>
  <key>CFBundleVersion</key><string>{version}</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>LSRequiresIPhoneOS</key><true/>
  <key>UILaunchScreen</key><dict/>
  <key>UIRequiredDeviceCapabilities</key><array><string>arm64</string></array>
{usage}</dict>
</plist>
"#,
        id = project.identifier(),
        name = xml_escape(project.app_name()),
        version = project.version(),
        usage = ios_usage_plist(&device_needs(&project.dir().join("forms")), aylis_build::used_classes(&project.dir().join("forms")).iter().any(|c| c == "SaveDialog"))
            + if push { "  <key>UIBackgroundModes</key><array><string>remote-notification</string></array>\n" } else { "" },
    );
    std::fs::write(app.join("Info.plist"), plist).map_err(|e| e.to_string())?;
    // O ícone como catálogo do Xcode (o `actool` só existe no Mac; fica pronto ao lado).
    if !project.publish.icon.is_empty() {
        let icon = crate::icons::Icon::open(&project.file(&project.publish.icon))?;
        crate::icons::ios(&icon, &out.join("Assets.xcassets"))?;
    }
    // O simulador aceita assinatura "ad hoc"; um iPhone exige o certificado do desenvolvedor.
    // O push (o `PushNotifications`): o direito `aps-environment`, sem o qual o iOS recusa o
    // registro (o de desenvolvimento; a App Store troca pelo de produção na assinatura).
    let mut sign = Command::new("codesign");
    sign.args(["--force", "--sign", "-"]);
    if push {
        let entitlements = out.join("entitlements.plist");
        std::fs::write(&entitlements, IOS_PUSH_ENTITLEMENTS).map_err(|e| e.to_string())?;
        sign.arg("--entitlements").arg(&entitlements);
    }
    run_tool(sign.arg(&app), control, sink, LineKind::Tool)?;
    sink(Event::Line(format!("{} (para o simulador; num iPhone, assine com o seu certificado)", app.display()), LineKind::Tool));
    Ok(Published { main: app, dir: out.to_path_buf() })
}

/// Os direitos do app de iOS com o `PushNotifications` (o APNs de desenvolvimento).
const IOS_PUSH_ENTITLEMENTS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>aps-environment</key><string>development</string>
</dict>
</plist>
"#;

// ---------------------------------------------------------------------------

/// Tamanho de um arquivo para mostrar (`3,4 MB`).
pub fn size(path: &Path) -> String {
    let bytes = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0) as f64;
    if bytes >= 1024.0 * 1024.0 {
        format!("{:.1} MB", bytes / 1024.0 / 1024.0).replace('.', ",")
    } else {
        format!("{:.0} KB", bytes / 1024.0)
    }
}

/// Algum form do projeto liga o traçado de raios do 3D (`"ray_traced": true`).
pub fn uses_ray_tracing(project: &Project) -> bool {
    project.forms.iter().any(|f| {
        std::fs::read_to_string(project.dir().join(&f.file)).is_ok_and(|text| {
            let compact: String = text.chars().filter(|c| !c.is_whitespace()).collect();
            compact.contains("\"ray_traced\":true")
        })
    })
}

/// O compilador DXC (`dxcompiler.dll` e, se estiver junto, o `dxil.dll`) que o traçado de
/// raios do 3D precisa no Direct3D 12: o de `AYLIS_DXC`, o do Vulkan SDK ou o do Windows SDK
/// (o mais novo), na mesma ordem em que o 3D procura.
pub fn dxc_files() -> Vec<PathBuf> {
    const FILE: &str = "dxcompiler.dll";
    let env = std::env::var_os("AYLIS_DXC").map(PathBuf::from);
    let tools = crate::dap::tools_dir().map(|d| d.join("dxc/bin/x64").join(FILE));
    let vulkan = std::env::var_os("VULKAN_SDK").map(|d| PathBuf::from(d).join("Bin").join(FILE));
    let kits = std::env::var_os("ProgramFiles(x86)").and_then(|p| {
        let mut found: Vec<PathBuf> = std::fs::read_dir(PathBuf::from(p).join("Windows Kits/10/bin")).ok()?.flatten().map(|e| e.path().join("x64").join(FILE)).filter(|p| p.is_file()).collect();
        found.sort();
        found.pop()
    });
    let Some(dxc) = [env, tools, vulkan, kits].into_iter().flatten().find(|p| p.is_file()) else { return Vec::new() };
    let dxil = dxc.with_file_name("dxil.dll");
    if dxil.is_file() { vec![dxc, dxil] } else { vec![dxc] }
}

/// No Windows, com o traçado de raios ligado em algum form: o DXC vai ao lado do
/// executável (sem ele, o app desenha o 3D sem raios). Devolve os arquivos copiados.
fn ray_tracing_dlls(project: &Project, dir: &Path, sink: Sink) -> Result<Vec<PathBuf>, String> {
    if !cfg!(windows) || !uses_ray_tracing(project) {
        return Ok(Vec::new());
    }
    let files = dxc_files();
    if files.is_empty() {
        sink(Event::Line("Traçado de raios: o compilador DXC (dxcompiler.dll) não foi achado; o app publicado desenha o 3D sem raios. Instale o Vulkan SDK ou o Windows SDK, ou aponte AYLIS_DXC para ele.".into(), LineKind::Tool));
        return Ok(Vec::new());
    }
    let mut copied = Vec::new();
    for f in files {
        let dest = dir.join(f.file_name().unwrap_or_default());
        std::fs::copy(&f, &dest).map_err(|e| format!("{}: {e}", f.display()))?;
        sink(Event::Line(format!("{} (traçado de raios, {})", dest.display(), size(&dest)), LineKind::Tool));
        copied.push(dest);
    }
    Ok(copied)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sem servidor, a página abria nos primeiros F5 e depois dava `ERR_FAILED` (conferência de
    /// 08/11): ela nunca ia para o cache. Agora entra na instalação, e o cache antigo só sai na
    /// ativação, com o novo pronto.
    #[test]
    fn the_service_worker_keeps_the_page_for_offline_use() {
        let sw = service_worker("0123456789", "app-0123456789.js", "app_bg-0123456789.wasm");
        assert!(!sw.contains('@'), "nada sem substituir: {sw}");
        assert!(sw.contains(r#"const PRECACHE = ["./", "./pkg/app-0123456789.js", "./pkg/app_bg-0123456789.wasm"];"#));
        let install = &sw[sw.find("\"install\"").unwrap()..sw.find("\"activate\"").unwrap()];
        assert!(install.contains("event.waitUntil") && install.contains("cache.put") && !install.contains("caches.delete"), "a instalação guarda e não apaga");
        assert!(sw[sw.find("\"activate\"").unwrap()..].contains("caches.delete"), "o antigo sai na ativação");
        // A navegação acha a página com query, com `index.html` e sem a barra final.
        assert!(sw.contains("key.search = \"\"") && sw.contains("/index.html") && sw.contains("Response.redirect"));
        assert!(sw.contains("response.redirected"), "resposta redirecionada não serve para navegação");
        // O JavaScript tem de ser válido (com o Node instalado).
        let file = std::env::temp_dir().join(format!("aylis_sw_{}.js", std::process::id()));
        std::fs::write(&file, &sw).unwrap();
        if let Ok(out) = Command::new("node").arg("--check").arg(&file).output() {
            assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        }
        let _ = std::fs::remove_file(&file);
    }

    #[test]
    fn device_components_declare_permissions_and_usage_texts() {
        let base = std::env::temp_dir().join(format!("aylis_device_needs_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let mut project = aylis_project::create(&base, "Fotos", Language::Rust).unwrap();
        let dir = project.dir().join("forms");
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        std::fs::write(
            dir.join("principal.json"),
            r#"{ "root": { "class": "Form", "children": [
                { "class": "Permissions", "permissions": "camera;location", "reasons": "camera=Para a foto & o perfil" },
                { "class": "CameraView" } ] } }"#,
        )
        .unwrap();
        std::fs::write(dir.join("sub/outro.json"), r#"{ "root": { "class": "Biometrics", "reason": "Entrar no app" } }"#).unwrap();
        let needs = device_needs(&dir);
        project.publish.android.permissions = vec!["CALL_PHONE".into(), "android.permission.CAMERA".into()];
        project.publish.android.deep_links = vec!["https://loja.com/pedido".into(), "fotos://".into()];
        project.publish.android.orientation = "portrait".into();
        std::fs::create_dir_all(project.dir().join("android")).unwrap();
        std::fs::write(project.file(ANDROID_MANIFEST_EXTRA), r#"<manifest xmlns:android="http://schemas.android.com/apk/res/android"><queries><package android:name="com.whatsapp" /></queries><application><meta-data android:name="x" android:value="1" /></application></manifest>"#).unwrap();
        let manifest = android_manifest(&project, "fotos", true).unwrap();
        assert!(manifest.contains("CALL_PHONE") && manifest.contains("screenOrientation=\"portrait\""), "{manifest}");
        assert!(manifest.contains("android:host=\"loja.com\"") && manifest.contains("android:scheme=\"fotos\"") && manifest.contains("autoVerify"));
        assert!(manifest.contains("<queries>") && manifest.contains("android:name=\"x\""), "o XML a mais entra");
        assert!(merge_manifest_extra(&manifest, "<manifest><queries>").is_err(), "XML quebrado dá erro");
        assert_eq!(manifest.matches("android.permission.CAMERA").count(), 1, "sem repetir: {manifest}");
        assert!(manifest.contains("ACCESS_FINE_LOCATION") && manifest.contains("ACCESS_COARSE_LOCATION"));
        assert!(manifest.contains("USE_BIOMETRIC"));
        assert!(manifest.contains("android.hardware.camera.any"));
        // Com os deep links, o link chega ao app aberto; sem AppLink, nem provedor nem SEND.
        assert!(manifest.contains("launchMode=\"singleTask\"") && !manifest.contains("aylis.files") && !manifest.contains("action.SEND"));
        // O AppLink com on_shared: o provedor dos arquivos e o app na lista de compartilhar.
        std::fs::write(dir.join("links.json"), r#"{ "root": { "class": "Form", "children": [ { "class": "AppLink", "name": "lnk", "on_shared": "recebido" } ] } }"#).unwrap();
        let manifest = android_manifest(&project, "fotos", true).unwrap();
        assert!(manifest.contains("AylisActivity$Files") && manifest.contains("android:authorities=\"dev.aylis.fotos.aylis.files\""), "{manifest}");
        assert!(manifest.contains("action.SEND\"") && manifest.contains("action.SEND_MULTIPLE"));
        // O BackgroundService: o serviço do tipo, as permissões e o boot (keep_alive).
        std::fs::write(dir.join("servico.json"), r#"{ "root": { "class": "Form", "children": [ { "class": "BackgroundService", "name": "svc", "kind": "location", "keep_alive": true } ] } }"#).unwrap();
        let manifest = android_manifest(&project, "fotos", true).unwrap();
        assert!(manifest.contains("AylisActivity$Background") && manifest.contains("foregroundServiceType=\"location\""), "{manifest}");
        assert!(manifest.contains("FOREGROUND_SERVICE_LOCATION") && manifest.contains("RECEIVE_BOOT_COMPLETED") && manifest.contains("AylisActivity$Boot"));
        // O Phone de app de telefone e a notificação de tela cheia.
        std::fs::write(dir.join("fone.json"), r#"{ "root": { "class": "Form", "children": [ { "class": "Phone", "name": "fone", "dialer_app": true }, { "class": "Notification", "name": "n", "full_screen": true } ] } }"#).unwrap();
        let manifest = android_manifest(&project, "fotos", true).unwrap();
        assert!(manifest.contains("AylisActivity$Calls") && manifest.contains("android.intent.action.DIAL") && manifest.contains("READ_CALL_LOG"), "{manifest}");
        assert!(manifest.contains("USE_FULL_SCREEN_INTENT") && manifest.contains("CALL_PHONE"));
        // O push do FCM: o receiver das mensagens, a permissão do c2dm e o Play Services visível.
        assert!(!manifest.contains("AylisActivity$Push"));
        std::fs::write(dir.join("push.json"), r#"{ "root": { "class": "Form", "children": [ { "class": "PushNotifications", "name": "push" } ] } }"#).unwrap();
        let manifest = android_manifest(&project, "fotos", true).unwrap();
        assert!(manifest.contains("AylisActivity$Push") && manifest.contains("com.google.android.c2dm.intent.RECEIVE") && manifest.contains("c2dm.permission.SEND"), "{manifest}");
        assert!(manifest.contains("com.google.android.c2dm.permission.RECEIVE") && manifest.contains("<package android:name=\"com.google.android.gms\" />"));
        assert!(manifest.contains("POST_NOTIFICATIONS"));
        // As ações: o receiver dos botões e a tela cheia do WakeAppAction.
        assert!(!manifest.contains("AylisActivity$Actions"));
        std::fs::write(dir.join("acoes.json"), r#"{ "root": { "class": "Form", "children": [ { "class": "PushTrigger", "name": "t" }, { "class": "WakeAppAction", "name": "w" } ] } }"#).unwrap();
        let manifest = android_manifest(&project, "fotos", true).unwrap();
        assert!(manifest.contains("AylisActivity$Actions") && manifest.contains("android.permission.VIBRATE"), "{manifest}");
        let plist = ios_usage_plist(&needs, true);
        assert!(plist.contains("<key>NSCameraUsageDescription</key><string>Para a foto &amp; o perfil</string>"), "{plist}");
        assert!(plist.contains("NSLocationWhenInUseUsageDescription"), "sem texto: um genérico");
        assert!(plist.contains("<key>NSFaceIDUsageDescription</key><string>Entrar no app</string>"));
        assert!(plist.contains("UIFileSharingEnabled"));
        let _ = std::fs::remove_dir_all(&base);
    }
}
