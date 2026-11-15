//! Assinatura de código no Windows (Authenticode, pelo `signtool.exe` do Windows SDK): o `.exe`
//! do app, o instalador do NSIS ou do Inno Setup e o `.msi`, com carimbo de tempo. Assinado,
//! o executável tem um fabricante verificável: o SmartScreen e os antivírus corporativos (o
//! Cortex XDR, o Defender) deixam de tratá-lo como de origem desconhecida.
//!
//! O certificado é da máquina, nunca do projeto: fica no `tools.json`
//! ([`crate::settings`], Opções da IDE → Assinatura), em uma de três origens:
//!
//! - `sign_thumbprint`: a impressão digital de um certificado do repositório do Windows
//!   (`/sha1`; o recomendado, inclusive o de token USB dos certificados EV);
//! - `sign_pfx`: um arquivo `.pfx`, com a senha só na variável [`PASSWORD_ENV`] (nunca gravada);
//! - `sign_dlib` e `sign_dmd`: a assinatura na nuvem (Azure Trusted Signing), a DLL do cliente
//!   e o JSON com a conta e o perfil.
//!
//! `sign_timestamp_url` troca o servidor de carimbo de tempo ([`DEFAULT_TIMESTAMP`]) e
//! `signtool` aponta o `signtool.exe` quando ele não está no PATH nem no Windows Kits.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::process::{Control, Event, LineKind, Sink, run_tool};

/// A impressão digital (SHA-1) do certificado no repositório do Windows.
pub const THUMBPRINT: &str = "sign_thumbprint";
/// O arquivo `.pfx`.
pub const PFX: &str = "sign_pfx";
/// A DLL do cliente de assinatura na nuvem (`Azure.CodeSigning.Dlib.dll`).
pub const DLIB: &str = "sign_dlib";
/// O JSON da assinatura na nuvem (`metadata.json`: o endereço, a conta e o perfil).
pub const DMD: &str = "sign_dmd";
/// O servidor de carimbo de tempo (RFC 3161).
pub const TIMESTAMP: &str = "sign_timestamp_url";
/// A variável com a senha do `.pfx` (nunca gravada em arquivo).
pub const PASSWORD_ENV: &str = "AYLIS_SIGN_PASSWORD";
/// O carimbo de tempo padrão (público, da DigiCert). Com ele, a assinatura continua válida
/// depois que o certificado vence.
pub const DEFAULT_TIMESTAMP: &str = "http://timestamp.digicert.com";

/// De onde vem o certificado.
#[derive(Clone, PartialEq, Eq)]
pub enum Source {
    /// Do repositório do Windows, pela impressão digital (40 dígitos hexadecimais).
    Store { thumbprint: String },
    /// De um `.pfx`; a senha vem da variável [`PASSWORD_ENV`] (só em memória).
    Pfx { file: PathBuf, password: Option<String> },
    /// Da nuvem (Azure Trusted Signing): `/dlib` e `/dmd`.
    Cloud { dlib: PathBuf, metadata: PathBuf },
}

// A senha nunca aparece num `{:?}` (log, erro, teste).
impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Store { thumbprint } => f.debug_struct("Store").field("thumbprint", thumbprint).finish(),
            Source::Pfx { file, password } => f.debug_struct("Pfx").field("file", file).field("password", &password.as_ref().map(|_| "*")).finish(),
            Source::Cloud { dlib, metadata } => f.debug_struct("Cloud").field("dlib", dlib).field("metadata", metadata).finish(),
        }
    }
}

/// A configuração de assinatura da máquina.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub source: Source,
    pub timestamp: String,
}

/// Só os dígitos hexadecimais, em maiúsculas: a impressão digital copiada do `certmgr` vem com
/// espaços e, às vezes, um caractere invisível (U+200E) no começo.
pub fn clean_thumbprint(text: &str) -> String {
    text.chars().filter(char::is_ascii_hexdigit).map(|c| c.to_ascii_uppercase()).collect()
}

impl Config {
    /// A configuração do `tools.json` (Opções da IDE), com a senha do `.pfx` da variável.
    /// `None`: nada configurado.
    pub fn from_settings() -> Option<Config> {
        Self::from_lookup(&|key| Some(crate::settings::tool_text(key)).filter(|s| !s.trim().is_empty()), std::env::var(PASSWORD_ENV).ok())
    }

    /// Como [`Config::from_settings`], de outro lugar (os testes). A ordem: a impressão
    /// digital, o `.pfx` e a nuvem (a primeira configurada vale).
    pub fn from_lookup(get: &dyn Fn(&str) -> Option<String>, password: Option<String>) -> Option<Config> {
        let get = |key: &str| get(key).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        let timestamp = get(TIMESTAMP).unwrap_or_else(|| DEFAULT_TIMESTAMP.to_string());
        let source = if let Some(thumbprint) = get(THUMBPRINT).map(|t| clean_thumbprint(&t)).filter(|t| !t.is_empty()) {
            Source::Store { thumbprint }
        } else if let Some(file) = get(PFX) {
            Source::Pfx { file: PathBuf::from(file), password: password.filter(|p| !p.is_empty()) }
        } else if let (Some(dlib), Some(metadata)) = (get(DLIB), get(DMD)) {
            Source::Cloud { dlib: PathBuf::from(dlib), metadata: PathBuf::from(metadata) }
        } else {
            return None;
        };
        Some(Config { source, timestamp })
    }

    /// O que impede de assinar (antes de chamar o `signtool`), em texto para o usuário.
    pub fn problem(&self) -> Option<String> {
        match &self.source {
            Source::Store { thumbprint } if thumbprint.len() != 40 => Some(format!("a impressão digital do certificado tem {} dígitos (são 40)", thumbprint.len())),
            Source::Store { .. } => None,
            Source::Pfx { file, .. } if !file.is_file() => Some(format!("o certificado {} não existe", file.display())),
            Source::Pfx { password: None, .. } => Some(format!("falta a senha do .pfx na variável {PASSWORD_ENV}")),
            Source::Pfx { .. } => None,
            Source::Cloud { dlib, .. } if !dlib.is_file() => Some(format!("a DLL da assinatura na nuvem {} não existe", dlib.display())),
            Source::Cloud { metadata, .. } if !metadata.is_file() => Some(format!("o arquivo da conta de assinatura {} não existe", metadata.display())),
            Source::Cloud { .. } => None,
        }
    }

    /// Em poucas palavras, para a verificação de ambiente e a publicação.
    pub fn describe(&self) -> String {
        match &self.source {
            Source::Store { thumbprint } => format!("certificado do Windows {thumbprint}"),
            Source::Pfx { file, .. } => format!("arquivo {}", file.display()),
            Source::Cloud { metadata, .. } => format!("na nuvem ({})", metadata.display()),
        }
    }

    /// Os argumentos do `signtool` para assinar `files`, sem o programa. A senha vai como está
    /// (o processo precisa dela); para mostrar, [`masked`].
    pub fn args(&self, files: &[PathBuf]) -> Vec<String> {
        let mut args: Vec<String> = ["sign", "/fd", "sha256", "/tr", self.timestamp.as_str(), "/td", "sha256"].map(str::to_string).to_vec();
        match &self.source {
            // `/sm` não vai: o `signtool` procura no repositório do usuário (o do token EV).
            Source::Store { thumbprint } => args.extend(["/sha1".to_string(), thumbprint.clone()]),
            Source::Pfx { file, password } => {
                args.extend(["/f".to_string(), file.display().to_string()]);
                if let Some(p) = password {
                    args.extend(["/p".to_string(), p.clone()]);
                }
            }
            Source::Cloud { dlib, metadata } => args.extend(["/dlib".to_string(), dlib.display().to_string(), "/dmd".to_string(), metadata.display().to_string()]),
        }
        args.extend(files.iter().map(|f| f.display().to_string()));
        args
    }
}

/// Os argumentos para mostrar: o que vem depois de `/p` vira `*`.
pub fn masked(args: &[String]) -> Vec<String> {
    let mut out = Vec::with_capacity(args.len());
    let mut hide = false;
    for a in args {
        out.push(if hide { "*".to_string() } else { a.clone() });
        hide = a.eq_ignore_ascii_case("/p");
    }
    out
}

/// O `signtool.exe`: o escolhido nas Opções (`signtool`), o do PATH ou o mais novo do Windows
/// Kits (`bin\<versão>\x64`, depois o `App Certification Kit`).
pub fn signtool() -> Option<PathBuf> {
    crate::settings::tool("signtool")
        .filter(|p| p.is_file())
        .or_else(|| crate::which("signtool").filter(|p| p.extension().is_some_and(|e| e.eq_ignore_ascii_case("exe"))))
        .or_else(windows_kits_signtool)
}

/// A pasta `bin` do Windows Kits 10.
pub fn windows_kits_bin() -> Option<PathBuf> {
    let base = std::env::var_os("ProgramFiles(x86)").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(r"C:\Program Files (x86)"));
    Some(base.join("Windows Kits").join("10").join("bin")).filter(|p| p.is_dir())
}

fn windows_kits_signtool() -> Option<PathBuf> {
    let bin = windows_kits_bin()?;
    let arch = if cfg!(target_arch = "aarch64") { "arm64" } else { "x64" };
    let mut versions: Vec<(Vec<u32>, PathBuf)> = std::fs::read_dir(&bin)
        .ok()?
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let numbers: Vec<u32> = name.split('.').map(|p| p.parse().ok()).collect::<Option<_>>()?;
            Some((numbers, e.path().join(arch).join("signtool.exe")))
        })
        .filter(|(_, p)| p.is_file())
        .collect();
    versions.sort();
    versions.pop().map(|(_, p)| p).or_else(|| Some(bin.parent()?.join("App Certification Kit").join("signtool.exe")).filter(|p| p.is_file()))
}

/// O certificado do repositório do Windows existe (no do usuário ou no da máquina)? `None`:
/// não deu para saber (sem PowerShell). A impressão digital já vem limpa (só hexadecimais),
/// então não há como ela virar comando.
pub fn store_has(thumbprint: &str) -> Option<bool> {
    if thumbprint.is_empty() || !thumbprint.chars().all(|c| c.is_ascii_hexdigit()) {
        return Some(false);
    }
    let script = format!("if ((Test-Path Cert:\\CurrentUser\\My\\{thumbprint}) -or (Test-Path Cert:\\LocalMachine\\My\\{thumbprint})) {{ 'sim' }} else {{ 'nao' }}");
    let mut command = Command::new("powershell");
    command.args(["-NoProfile", "-NonInteractive", "-Command", &script]);
    let out = crate::env::output_within(crate::process::quiet(&mut command), crate::env::PROCESS_LIMIT).ok()?;
    match String::from_utf8_lossy(&out.stdout).trim() {
        "sim" => Some(true),
        "nao" => Some(false),
        _ => None,
    }
}

/// O que a publicação faz com a assinatura.
#[derive(Debug, PartialEq, Eq)]
pub enum Plan {
    /// O projeto pediu para não assinar (`publish.windows.skip_signing`).
    Skip,
    /// Sem certificado na máquina: sai sem assinatura, com um aviso.
    Unsigned,
    Sign(Config),
}

/// A decisão, sem efeito colateral (os testes conferem): o projeto e a configuração da máquina.
pub fn plan(skip: bool, config: Option<Config>) -> Plan {
    match (skip, config) {
        (true, _) => Plan::Skip,
        (false, None) => Plan::Unsigned,
        (false, Some(c)) => Plan::Sign(c),
    }
}

/// O aviso de quando falta o certificado (uma linha por publicação).
pub const UNSIGNED_WARNING: &str = "⚠ o executável sai sem assinatura: o SmartScreen e os antivírus corporativos podem bloqueá-lo. \
     Configure o certificado em Ferramentas → Opções → Assinatura (veja docs/publicar-windows.md).";

/// Assina os arquivos publicados do Windows conforme o projeto e a máquina. Sem certificado,
/// só avisa (não é erro); com certificado, erro do `signtool` é erro da publicação.
pub fn sign_published(skip: bool, files: &[PathBuf], control: &Control, sink: Sink) -> Result<(), String> {
    sign_published_with(skip, Config::from_settings(), files, control, sink)
}

/// Como [`sign_published`], com a configuração dada (os testes).
pub fn sign_published_with(skip: bool, config: Option<Config>, files: &[PathBuf], control: &Control, sink: Sink) -> Result<(), String> {
    apply(&plan(skip, config), files, true, control, sink)
}

/// Cumpre o plano para `files`. `warn`: sem certificado, avisa (a publicação passa `true` só
/// no primeiro passo, o `.exe`, para o aviso sair uma vez).
pub fn apply(plan: &Plan, files: &[PathBuf], warn: bool, control: &Control, sink: Sink) -> Result<(), String> {
    match plan {
        Plan::Skip => Ok(()),
        Plan::Unsigned => {
            if warn {
                sink(Event::Line(UNSIGNED_WARNING.into(), LineKind::Tool));
            }
            Ok(())
        }
        Plan::Sign(config) => sign(config, files, control, sink),
    }
}

/// Assina `files` com o `signtool` (um processo só para todos, com argumentos separados).
pub fn sign(config: &Config, files: &[PathBuf], control: &Control, sink: Sink) -> Result<(), String> {
    if files.is_empty() {
        return Ok(());
    }
    if let Some(problem) = config.problem() {
        return Err(format!("assinatura: {problem}"));
    }
    let tool = signtool().ok_or("assinatura: signtool.exe não encontrado (instale o Windows SDK ou aponte o signtool nas Opções)")?;
    sink(Event::Step(format!("Assinando ({})…", config.describe())));
    let args = config.args(files);
    sink(Event::Line(format!("signtool {}", masked(&args).join(" ")), LineKind::Tool));
    run_tool(Command::new(&tool).args(&args), control, sink, LineKind::Tool).map_err(|e| format!("assinatura: {e}"))
}

/// O item da verificação de ambiente: `Ok(detalhe)` se dá para assinar, `Err(o que falta)`.
pub fn environment() -> Result<String, String> {
    let config = Config::from_settings().ok_or("nenhum certificado configurado (Opções → Assinatura): os executáveis saem sem assinatura")?;
    let tool = signtool().ok_or("signtool.exe não encontrado (vem com o Windows SDK)")?;
    if let Some(problem) = config.problem() {
        return Err(problem);
    }
    if let Source::Store { thumbprint } = &config.source {
        if store_has(thumbprint) == Some(false) {
            return Err(format!("o certificado {thumbprint} não está no repositório do Windows (Pessoal)"));
        }
    }
    Ok(format!("{} ({})", config.describe(), tool.display()))
}

/// Os arquivos que a publicação do Windows assina, na ordem: o `.exe` do app e o instalador.
pub fn published_files(exe: &Path, installer: Option<&Path>) -> Vec<PathBuf> {
    std::iter::once(exe).chain(installer).map(Path::to_path_buf).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lookup(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        move |k| pairs.iter().find(|(key, _)| *key == k).map(|(_, v)| v.to_string())
    }

    /// A linha de comando do `signtool` de cada origem (sem executar): SHA-256 no arquivo e no
    /// carimbo, o carimbo configurável e os arquivos no fim.
    #[test]
    fn the_signtool_command_line_of_each_source() {
        let files = [PathBuf::from(r"C:\dist\App.exe"), PathBuf::from(r"C:\dist\App-1.0.0.msi")];
        // O repositório do Windows: a impressão digital limpa (o certmgr põe espaços e U+200E).
        let c = Config::from_lookup(&lookup(&[(THUMBPRINT, "\u{200e}ab cd ef 01 23 45 67 89 ab cd ef 01 23 45 67 89 ab cd ef 01")]), None).unwrap();
        assert_eq!(c.source, Source::Store { thumbprint: "ABCDEF0123456789ABCDEF0123456789ABCDEF01".into() });
        assert_eq!(c.problem(), None);
        assert_eq!(
            c.args(&files).join(" "),
            r"sign /fd sha256 /tr http://timestamp.digicert.com /td sha256 /sha1 ABCDEF0123456789ABCDEF0123456789ABCDEF01 C:\dist\App.exe C:\dist\App-1.0.0.msi"
        );
        // O .pfx, com a senha da variável e outro carimbo.
        let c = Config::from_lookup(&lookup(&[(PFX, r"C:\certs\empresa.pfx"), (TIMESTAMP, "http://timestamp.sectigo.com")]), Some("s3gr3do".into())).unwrap();
        let args = c.args(&files[..1]);
        assert_eq!(args.join(" "), r"sign /fd sha256 /tr http://timestamp.sectigo.com /td sha256 /f C:\certs\empresa.pfx /p s3gr3do C:\dist\App.exe");
        // A senha nunca aparece no que se mostra.
        let shown = masked(&args).join(" ");
        assert!(!shown.contains("s3gr3do") && shown.contains("/p * "), "{shown}");
        assert!(!format!("{c:?}").contains("s3gr3do"), "nem no Debug");
        // Sem a senha, o .pfx não serve (antes de rodar o signtool).
        let c = Config::from_lookup(&lookup(&[(PFX, r"C:\nao\existe.pfx")]), None).unwrap();
        assert!(c.problem().is_some());
        // A nuvem (Azure Trusted Signing).
        let c = Config::from_lookup(&lookup(&[(DLIB, r"C:\ts\Azure.CodeSigning.Dlib.dll"), (DMD, r"C:\ts\metadata.json")]), None).unwrap();
        assert_eq!(
            c.args(&files[..1]).join(" "),
            r"sign /fd sha256 /tr http://timestamp.digicert.com /td sha256 /dlib C:\ts\Azure.CodeSigning.Dlib.dll /dmd C:\ts\metadata.json C:\dist\App.exe"
        );
        // A impressão digital vale antes do .pfx; nada configurado, nada.
        let c = Config::from_lookup(&lookup(&[(PFX, "a.pfx"), (THUMBPRINT, "12")]), None).unwrap();
        assert!(matches!(c.source, Source::Store { .. }));
        assert!(c.problem().unwrap().contains("40"), "impressão digital curta");
        assert_eq!(Config::from_lookup(&lookup(&[(TIMESTAMP, "http://x")]), None), None);
    }

    /// Sem certificado, a publicação só avisa (não falha); o projeto pode desligar.
    #[test]
    fn publishing_without_a_certificate_only_warns() {
        assert_eq!(plan(false, None), Plan::Unsigned);
        assert_eq!(plan(true, None), Plan::Skip);
        let config = Config::from_lookup(&lookup(&[(THUMBPRINT, "ABCDEF0123456789ABCDEF0123456789ABCDEF01")]), None);
        assert!(matches!(plan(false, config.clone()), Plan::Sign(_)));
        assert_eq!(plan(true, config), Plan::Skip);
        // O caminho todo, sem certificado configurado: uma linha de aviso e Ok.
        let control = Control::new();
        let files = [PathBuf::from("App.exe"), PathBuf::from("App-setup.exe")];
        let mut lines = Vec::new();
        let result = sign_published_with(false, None, &files, &control, &mut |e| {
            if let Event::Line(l, _) = e {
                lines.push(l)
            }
        });
        assert!(result.is_ok());
        assert_eq!(lines.len(), 1, "um aviso só, para o .exe e o instalador");
        assert!(lines[0].contains("sem assinatura"));
        // Desligado no projeto: nem o aviso.
        assert!(sign_published_with(true, None, &files, &control, &mut |_| panic!("sem aviso")).is_ok());
        // Assinar nada não chama o signtool.
        let c = Config::from_lookup(&lookup(&[(THUMBPRINT, "ABCDEF0123456789ABCDEF0123456789ABCDEF01")]), None).unwrap();
        assert!(sign(&c, &[], &control, &mut |_| panic!("nada a assinar")).is_ok());
        assert_eq!(published_files(Path::new("a.exe"), Some(Path::new("a.msi"))), [PathBuf::from("a.exe"), PathBuf::from("a.msi")]);
    }
}
