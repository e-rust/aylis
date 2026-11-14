//! Os instaladores `.exe` do Windows (Publicar → Windows → instalador): NSIS e Inno Setup, com
//! as opções de `publish.windows` do projeto. O `.msi` (WiX) fica em `publish.rs`.
//!
//! Os dois instalam por usuário (sem pedir administrador), com atalho no menu Iniciar,
//! desinstalação em Aplicativos e os dados de versão no próprio instalador (sem eles o Windows
//! Defender marcava instaladores como `Program:Win32/Contebrew.A!ml`). Opcionais, do projeto:
//! iniciar com o Windows, atalho na área de trabalho, "Abrir como administrador" (só a escolha,
//! no registro, para o app ler), fechar o app aberto, abrir no fim e perguntar se apaga a pasta
//! de dados ao desinstalar.

use std::path::{Path, PathBuf};
use std::process::Command;

use aylis_project::{Project, WindowsOptions};

use crate::process::{Control, Event, LineKind, Sink, run_tool};

/// Os arquivos de `publish.windows.files` (aceita `pasta/*.dll`), copiados para `out`, ao lado
/// do executável. Devolve as cópias.
pub(super) fn copy_extra_files(project: &Project, out: &Path, sink: Sink) -> Result<Vec<PathBuf>, String> {
    let mut copied = Vec::new();
    for pattern in &project.publish.windows.files {
        let path = project.file(pattern);
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let sources: Vec<PathBuf> = match name.split_once('*') {
            Some((prefix, suffix)) => {
                let dir = path.parent().ok_or_else(|| format!("{pattern}: sem pasta"))?;
                let (prefix, suffix) = (prefix.to_lowercase(), suffix.to_lowercase());
                let mut list: Vec<PathBuf> = std::fs::read_dir(dir)
                    .map_err(|e| format!("{}: {e}", dir.display()))?
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| {
                        p.is_file()
                            && p.file_name().is_some_and(|n| {
                                let n = n.to_string_lossy().to_lowercase();
                                n.len() >= prefix.len() + suffix.len() && n.starts_with(&prefix) && n.ends_with(&suffix)
                            })
                    })
                    .collect();
                list.sort();
                list
            }
            None => vec![path],
        };
        if sources.is_empty() {
            return Err(format!("publish.windows.files: nenhum arquivo em {pattern}"));
        }
        for src in sources {
            let dst = out.join(src.file_name().ok_or_else(|| format!("{}: sem nome", src.display()))?);
            std::fs::copy(&src, &dst).map_err(|e| format!("{}: {e}", src.display()))?;
            copied.push(dst);
        }
    }
    if !copied.is_empty() {
        sink(Event::Line(format!("{} arquivo(s) ao lado do executável", copied.len()), LineKind::Tool));
    }
    Ok(copied)
}

/// A versão com 4 partes numéricas (`0.52.7-beta` → `0.52.7.0`), como os dados de versão pedem.
pub(super) fn version4(version: &str) -> String {
    let mut parts: Vec<u32> = version
        .split('.')
        .take(4)
        .map(|p| p.chars().take_while(char::is_ascii_digit).collect::<String>().parse::<u32>().unwrap_or(0).min(65535))
        .collect();
    parts.resize(4, 0);
    parts.iter().map(u32::to_string).collect::<Vec<_>>().join(".")
}

/// Os marcadores de pasta do projeto (`{localappdata}`...) com as variáveis de cada instalador.
fn expand(path: &str, nsis: bool) -> String {
    let pairs: [(&str, &str, &str); 4] = [
        ("{localappdata}", "$LOCALAPPDATA", "{localappdata}"),
        ("{appdata}", "$APPDATA", "{userappdata}"),
        ("{programfiles}", "$PROGRAMFILES64", "{autopf}"),
        ("{documents}", "$DOCUMENTS", "{userdocs}"),
    ];
    let mut out = path.replace('/', "\\");
    for (mark, n, i) in pairs {
        out = out.replace(mark, if nsis { n } else { i });
    }
    out
}

/// Os textos do instalador, no idioma do projeto.
struct Texts {
    /// `pt`, `es` ou `en`.
    code: &'static str,
    autostart: &'static str,
    desktop: &'static str,
    run_as_admin: &'static str,
    running: &'static str,
    /// O app continuou aberto depois do `taskkill` (aberto como administrador, por exemplo).
    still_running: &'static str,
    remove_data: &'static str,
    need_64: &'static str,
    description: &'static str,
}

fn texts(locale: &str) -> Texts {
    match locale.split(['-', '_']).next().unwrap_or("").to_ascii_lowercase().as_str() {
        "pt" => Texts {
            code: "pt",
            autostart: "Iniciar com o Windows",
            desktop: "Atalho na área de trabalho",
            run_as_admin: "Abrir como administrador",
            running: "está aberto. Clique em OK para fechar e continuar.",
            still_running: "continua aberto e não pôde ser fechado (talvez esteja rodando como administrador). Feche-o e tente de novo.",
            remove_data: "Apagar também as configurações e os dados?",
            need_64: "precisa do Windows de 64 bits.",
            description: "Instalador do",
        },
        "es" => Texts {
            code: "es",
            autostart: "Iniciar con Windows",
            desktop: "Acceso directo en el escritorio",
            run_as_admin: "Abrir como administrador",
            running: "está abierto. Haga clic en Aceptar para cerrarlo y continuar.",
            still_running: "sigue abierto y no se pudo cerrar (quizás se ejecuta como administrador). Ciérrelo e inténtelo de nuevo.",
            remove_data: "¿Borrar también la configuración y los datos?",
            need_64: "necesita Windows de 64 bits.",
            description: "Instalador de",
        },
        _ => Texts {
            code: "en",
            autostart: "Start with Windows",
            desktop: "Desktop shortcut",
            run_as_admin: "Run as administrator",
            running: "is open. Click OK to close it and continue.",
            still_running: "is still open and could not be closed (it may be running as administrator). Close it and try again.",
            remove_data: "Also delete the settings and data?",
            need_64: "needs 64-bit Windows.",
            description: "Installer for",
        },
    }
}

/// O que os dois instaladores precisam do projeto.
struct Plan<'a> {
    app: String,
    publisher: String,
    version: String,
    version4: String,
    w: &'a WindowsOptions,
    exe: PathBuf,
    exe_name: String,
    extra: Vec<PathBuf>,
    icon: Option<PathBuf>,
    texts: Texts,
    /// O nome do instalador, sem a pasta (`App-1.0.0-setup.exe`).
    out_name: String,
    /// Os prefixos dos `UrlScheme` ligados (prefixo e descrição): registrados e tirados na
    /// desinstalação.
    schemes: Vec<(String, String)>,
}

impl<'a> Plan<'a> {
    fn new(project: &'a Project, exe: &Path, extra: &[PathBuf]) -> Self {
        let app = project.app_name().to_string();
        let publisher = if project.publish.publisher.is_empty() { app.clone() } else { project.publish.publisher.clone() };
        let icon = Some(project.dir().join(aylis_build::APP_ICON)).filter(|p| p.is_file());
        Plan {
            out_name: format!("{}-{}-setup.exe", crate::file_name(&app), project.version()),
            version: project.version().to_string(),
            version4: version4(project.version()),
            w: &project.publish.windows,
            exe_name: exe.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
            exe: exe.to_path_buf(),
            extra: extra.to_vec(),
            icon,
            texts: texts(&project.locale),
            schemes: super::url_schemes(project).into_iter().map(|(s, d)| (s, if d.is_empty() { app.clone() } else { d })).collect(),
            app,
            publisher,
        }
    }

    fn install_dir(&self, nsis: bool) -> String {
        let dir = if self.w.install_dir.is_empty() { format!("{{localappdata}}\\Programs\\{}", self.app) } else { self.w.install_dir.clone() };
        expand(&dir, nsis)
    }

    /// A pasta do executável dentro da de instalação (`bin` ou vazia), com `\` no fim se houver.
    fn exe_sub(&self) -> String {
        let sub = self.w.exe_dir.trim_matches(['/', '\\']).replace('/', "\\");
        if sub.is_empty() { sub } else { format!("{sub}\\") }
    }

    fn offers(&self, option: &str) -> Option<bool> {
        match option.to_ascii_lowercase().as_str() {
            "on" | "true" | "yes" => Some(true),
            "off" | "false" | "no" => Some(false),
            _ => None,
        }
    }
}

/// Texto entre aspas no NSIS (`$\"` para as aspas). O `$` fica: os caminhos usam as variáveis
/// do NSIS (`$SMPROGRAMS`), e os textos fixos não têm cifrão.
fn nsis_str(text: &str) -> String {
    format!("\"{}\"", text.replace('"', "$\\\""))
}

/// Gera e compila o instalador do NSIS em `out`.
pub(super) fn nsis(project: &Project, exe: &Path, extra: &[PathBuf], out: &Path, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    let makensis = makensis().ok_or("NSIS não encontrado (instale de nsis.sourceforge.io ou defina NSIS_HOME)")?;
    sink(Event::Step("Montando o instalador (NSIS)…".into()));
    let p = Plan::new(project, exe, extra);
    let target = out.join(&p.out_name);
    let script = nsis_script(&p, &target);
    let path = out.join("instalador.nsi");
    // UTF-8 com BOM: o NSIS lê assim os acentos dos textos.
    std::fs::write(&path, format!("\u{feff}{script}")).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&target);
    run_tool(Command::new(&makensis).arg("/V2").arg(&path), control, sink, LineKind::Tool)?;
    let _ = std::fs::remove_file(&path);
    if !target.is_file() {
        return Err("o NSIS terminou sem gerar o instalador".into());
    }
    sink(Event::Line(format!("{} ({})", target.display(), super::size(&target)), LineKind::Tool));
    Ok(target)
}

fn nsis_script(p: &Plan, target: &Path) -> String {
    let t = &p.texts;
    let app = &p.app;
    let (lang, lang_id) = match t.code {
        "pt" => ("PortugueseBR", "LANG_PORTUGUESEBR"),
        "es" => ("Spanish", "LANG_SPANISH"),
        _ => ("English", "LANG_ENGLISH"),
    };
    let exe_dir = format!("$INSTDIR\\{}", p.exe_sub());
    let exe_dir = exe_dir.trim_end_matches('\\').to_string();
    let exe_target = format!("{exe_dir}\\{}", p.exe_name);
    let uninst = format!("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{app}");
    let run_key = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let autostart = p.offers(&p.w.autostart);
    let desktop = p.offers(&p.w.desktop_shortcut);
    let admin = p.offers(&p.w.run_as_admin);
    let choices = autostart.is_some() || desktop.is_some() || admin.is_some();
    let mut s = String::new();
    let mut line = |text: &str| {
        s.push_str(text);
        s.push('\n');
    };
    line("; Gerado pelo Aylis (Publicar → Windows → instalador NSIS). Não edite: refeito a cada publicação.");
    line("Unicode True");
    line("!include \"MUI2.nsh\"");
    line("!include \"LogicLib.nsh\"");
    line("!include \"Sections.nsh\"");
    line("!include \"x64.nsh\"");
    line(&format!("Name {}", nsis_str(app)));
    line(&format!("OutFile {}", nsis_str(&target.display().to_string())));
    line(&format!("InstallDir \"{}\"", p.install_dir(true)));
    line(&format!("InstallDirRegKey HKCU {} \"InstallDir\"", nsis_str(&format!("Software\\{app}"))));
    line("RequestExecutionLevel user");
    line("SetCompressor /SOLID lzma");
    if let Some(icon) = &p.icon {
        line(&format!("!define MUI_ICON {}", nsis_str(&icon.display().to_string())));
        line(&format!("!define MUI_UNICON {}", nsis_str(&icon.display().to_string())));
    }
    line("!define MUI_ABORTWARNING");
    line("!insertmacro MUI_PAGE_DIRECTORY");
    if choices {
        line("!insertmacro MUI_PAGE_COMPONENTS");
    }
    line("!insertmacro MUI_PAGE_INSTFILES");
    line("!insertmacro MUI_UNPAGE_CONFIRM");
    line("!insertmacro MUI_UNPAGE_INSTFILES");
    line(&format!("!insertmacro MUI_LANGUAGE \"{lang}\""));
    // Os dados de versão (Propriedades → Detalhes do instalador).
    line(&format!("VIProductVersion \"{}\"", p.version4));
    line(&format!("VIFileVersion \"{}\"", p.version4));
    for (key, value) in [
        ("ProductName", app.clone()),
        ("CompanyName", p.publisher.clone()),
        ("FileDescription", format!("{} {app}", t.description)),
        ("FileVersion", p.version.clone()),
        ("ProductVersion", p.version.clone()),
        ("LegalCopyright", p.publisher.clone()),
        ("OriginalFilename", p.out_name.clone()),
    ] {
        line(&format!("VIAddVersionKey /LANG=${{{lang_id}}} \"{key}\" {}", nsis_str(&value)));
    }
    if p.w.close_running {
        // O app aberto segura o executável: pergunta e fecha (pelo processo, que é o que
        // segura o arquivo).
        line("!macro AYLIS_CLOSE_APP UN");
        line("Function ${UN}AylisCloseApp");
        line(&format!(
            "  nsExec::ExecToStack '\"$SYSDIR\\cmd.exe\" /c tasklist /FI \"IMAGENAME eq {0}\" /NH | \"$SYSDIR\\findstr.exe\" /I /C:\"{0}\"'",
            p.exe_name
        ));
        line("  Pop $0");
        line("  Pop $1");
        line("  ${If} $0 == 0");
        line(&format!("    MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION {} IDOK +2", nsis_str(&format!("{app} {}", t.running))));
        line("    Abort");
        line(&format!("    nsExec::Exec 'taskkill /F /IM \"{}\" /T'", p.exe_name));
        line("    Pop $0");
        line("    Sleep 800");
        // Continua aberto (o Windows nega encerrar um app de administrador a quem não é): avisa
        // e para, em vez de seguir e falhar ao gravar o executável.
        line(&format!(
            "    nsExec::ExecToStack '\"$SYSDIR\\cmd.exe\" /c tasklist /FI \"IMAGENAME eq {0}\" /NH | \"$SYSDIR\\findstr.exe\" /I /C:\"{0}\"'",
            p.exe_name
        ));
        line("    Pop $0");
        line("    Pop $1");
        line("    ${If} $0 == 0");
        line(&format!("      MessageBox MB_OK|MB_ICONSTOP {}", nsis_str(&format!("{app} {}", t.still_running))));
        line("      Abort");
        line("    ${EndIf}");
        line("  ${EndIf}");
        line("FunctionEnd");
        line("!macroend");
        line("!insertmacro AYLIS_CLOSE_APP \"\"");
        line("!insertmacro AYLIS_CLOSE_APP \"un.\"");
    }
    line("Function .onInit");
    line("  ${IfNot} ${RunningX64}");
    line(&format!("    MessageBox MB_OK|MB_ICONSTOP {}", nsis_str(&format!("{app} {}", t.need_64))));
    line("    Abort");
    line("  ${EndIf}");
    if p.w.close_running {
        line("  Call AylisCloseApp");
    }
    if choices {
        line("  Call AylisKeepChoices");
    }
    line("FunctionEnd");
    line(&format!("Section {} SecMain", nsis_str(app)));
    line("  SectionIn RO");
    line(&format!("  SetOutPath \"{exe_dir}\""));
    line(&format!("  File {}", nsis_str(&p.exe.display().to_string())));
    for f in &p.extra {
        line(&format!("  File {}", nsis_str(&f.display().to_string())));
    }
    line("  SetOutPath \"$INSTDIR\"");
    line("  WriteUninstaller \"$INSTDIR\\uninstall.exe\"");
    line(&format!("  CreateDirectory {}", nsis_str(&format!("$SMPROGRAMS\\{app}"))));
    line(&format!("  CreateShortcut {} \"{exe_target}\"", nsis_str(&format!("$SMPROGRAMS\\{app}\\{app}.lnk"))));
    line(&format!("  WriteRegStr HKCU {} \"InstallDir\" \"$INSTDIR\"", nsis_str(&format!("Software\\{app}"))));
    for (name, value) in [("DisplayName", nsis_str(app)), ("DisplayVersion", nsis_str(&p.version)), ("Publisher", nsis_str(&p.publisher)), ("DisplayIcon", format!("\"{exe_target}\""))] {
        line(&format!("  WriteRegStr HKCU {} \"{name}\" {value}", nsis_str(&uninst)));
    }
    line(&format!("  WriteRegStr HKCU {} \"UninstallString\" '\"$INSTDIR\\uninstall.exe\"'", nsis_str(&uninst)));
    line(&format!("  WriteRegDWORD HKCU {} \"NoModify\" 1", nsis_str(&uninst)));
    line(&format!("  WriteRegDWORD HKCU {} \"NoRepair\" 1", nsis_str(&uninst)));
    for (scheme, description) in &p.schemes {
        let key = format!("Software\\Classes\\{scheme}");
        line(&format!("  WriteRegStr HKCU {} \"\" {}", nsis_str(&key), nsis_str(&format!("URL:{description}"))));
        line(&format!("  WriteRegStr HKCU {} \"URL Protocol\" \"\"", nsis_str(&key)));
        line(&format!("  WriteRegStr HKCU {} \"\" '\"{exe_target}\",0'", nsis_str(&format!("{key}\\DefaultIcon"))));
        line(&format!("  WriteRegStr HKCU {} \"\" '\"{exe_target}\" \"%1\"'", nsis_str(&format!("{key}\\shell\\open\\command"))));
    }
    line("SectionEnd");
    if let Some(on) = autostart {
        let args = if p.w.autostart_args.is_empty() { String::new() } else { format!(" {}", p.w.autostart_args) };
        line(&format!("Section {}{} SecAutostart", if on { "" } else { "/o " }, nsis_str(t.autostart)));
        line(&format!("  WriteRegStr HKCU \"{run_key}\" {} '\"{exe_target}\"{args}'", nsis_str(app)));
        line("SectionEnd");
        // Desmarcado: tira a entrada de uma instalação anterior.
        line("Section \"-AylisAutostartOff\"");
        line("  ${IfNot} ${SectionIsSelected} ${SecAutostart}");
        line(&format!("    DeleteRegValue HKCU \"{run_key}\" {}", nsis_str(app)));
        line("  ${EndIf}");
        line("SectionEnd");
    }
    if let Some(on) = desktop {
        line(&format!("Section {}{} SecDesktop", if on { "" } else { "/o " }, nsis_str(t.desktop)));
        line(&format!("  CreateShortcut {} \"{exe_target}\"", nsis_str(&format!("$DESKTOP\\{app}.lnk"))));
        line("SectionEnd");
    }
    if let Some(on) = admin {
        // Só a escolha, para o app ler: o instalador segue sem pedir administrador.
        let key = nsis_str(&format!("Software\\{app}"));
        line(&format!("Section {}{} SecRunAsAdmin", if on { "" } else { "/o " }, nsis_str(t.run_as_admin)));
        line(&format!("  WriteRegDWORD HKCU {key} \"RunAsAdmin\" 1"));
        line("SectionEnd");
        line("Section \"-AylisRunAsAdminOff\"");
        line("  ${IfNot} ${SectionIsSelected} ${SecRunAsAdmin}");
        line(&format!("    WriteRegDWORD HKCU {key} \"RunAsAdmin\" 0"));
        line("  ${EndIf}");
        line("SectionEnd");
    }
    if choices {
        // Atualização (já instalado): as opções vêm marcadas como o usuário deixou, não no
        // padrão de novo (quem desligou o iniciar com o Windows não o vê religado). Depois
        // das seções, que definem os ${Sec...}.
        line("Function AylisKeepChoices");
        line(&format!("  ReadRegStr $0 HKCU {} \"InstallDir\"", nsis_str(&format!("Software\\{app}"))));
        line("  ${If} $0 != \"\"");
        if autostart.is_some() {
            line(&format!("    ReadRegStr $1 HKCU \"{run_key}\" {}", nsis_str(app)));
            line("    ${If} $1 == \"\"");
            line("      !insertmacro UnselectSection ${SecAutostart}");
            line("    ${Else}");
            line("      !insertmacro SelectSection ${SecAutostart}");
            line("    ${EndIf}");
        }
        if desktop.is_some() {
            line(&format!("    ${{If}} ${{FileExists}} {}", nsis_str(&format!("$DESKTOP\\{app}.lnk"))));
            line("      !insertmacro SelectSection ${SecDesktop}");
            line("    ${Else}");
            line("      !insertmacro UnselectSection ${SecDesktop}");
            line("    ${EndIf}");
        }
        if admin.is_some() {
            // Sem o valor (instalado por uma versão sem a opção): fica o padrão do projeto.
            line("    ClearErrors");
            line(&format!("    ReadRegDWORD $1 HKCU {} \"RunAsAdmin\"", nsis_str(&format!("Software\\{app}"))));
            line("    ${IfNot} ${Errors}");
            line("      ${If} $1 == 1");
            line("        !insertmacro SelectSection ${SecRunAsAdmin}");
            line("      ${Else}");
            line("        !insertmacro UnselectSection ${SecRunAsAdmin}");
            line("      ${EndIf}");
            line("    ${EndIf}");
        }
        line("  ${EndIf}");
        line("FunctionEnd");
    }
    if p.w.run_after_install {
        line("Function .onInstSuccess");
        line(&format!("  Exec '\"{exe_target}\"'"));
        line("FunctionEnd");
    }
    if p.w.close_running {
        line("Function un.onInit");
        line("  Call un.AylisCloseApp");
        line("FunctionEnd");
    }
    line("Section \"Uninstall\"");
    line(&format!("  Delete \"{exe_target}\""));
    for f in &p.extra {
        if let Some(name) = f.file_name() {
            line(&format!("  Delete \"{exe_dir}\\{}\"", name.to_string_lossy()));
        }
    }
    line(&format!("  RMDir \"{exe_dir}\""));
    line("  Delete \"$INSTDIR\\uninstall.exe\"");
    line("  RMDir \"$INSTDIR\"");
    line(&format!("  Delete {}", nsis_str(&format!("$SMPROGRAMS\\{app}\\{app}.lnk"))));
    line(&format!("  RMDir {}", nsis_str(&format!("$SMPROGRAMS\\{app}"))));
    line(&format!("  Delete {}", nsis_str(&format!("$DESKTOP\\{app}.lnk"))));
    line(&format!("  DeleteRegValue HKCU \"{run_key}\" {}", nsis_str(app)));
    line(&format!("  DeleteRegKey HKCU {}", nsis_str(&uninst)));
    line(&format!("  DeleteRegKey HKCU {}", nsis_str(&format!("Software\\{app}"))));
    for (scheme, _) in &p.schemes {
        line(&format!("  DeleteRegKey HKCU {}", nsis_str(&format!("Software\\Classes\\{scheme}"))));
    }
    if !p.w.data_dir.is_empty() {
        line(&format!("  MessageBox MB_YESNO|MB_ICONQUESTION {} IDNO +2", nsis_str(t.remove_data)));
        line(&format!("    RMDir /r \"{}\"", expand(&p.w.data_dir, true)));
    }
    line("SectionEnd");
    s
}

/// Texto entre aspas no Inno (as aspas dobradas).
fn inno_str(text: &str) -> String {
    format!("\"{}\"", text.replace('"', "\"\""))
}

/// Gera e compila o instalador do Inno Setup em `out`.
pub(super) fn inno(project: &Project, exe: &Path, extra: &[PathBuf], out: &Path, control: &Control, sink: Sink) -> Result<PathBuf, String> {
    let iscc = iscc().ok_or("Inno Setup não encontrado (instale de jrsoftware.org/isinfo.php; o ISCC.exe vem junto)")?;
    sink(Event::Step("Montando o instalador (Inno Setup)…".into()));
    let p = Plan::new(project, exe, extra);
    let target = out.join(&p.out_name);
    let script = inno_script(&p, out, &super::stable_guid(&project.identifier()));
    let path = out.join("instalador.iss");
    // UTF-8 com BOM: o Inno lê assim os acentos dos textos.
    std::fs::write(&path, format!("\u{feff}{script}")).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&target);
    run_tool(Command::new(&iscc).arg("/Qp").arg(&path), control, sink, LineKind::Tool)?;
    let _ = std::fs::remove_file(&path);
    if !target.is_file() {
        return Err("o Inno Setup terminou sem gerar o instalador".into());
    }
    sink(Event::Line(format!("{} ({})", target.display(), super::size(&target)), LineKind::Tool));
    Ok(target)
}

fn inno_script(p: &Plan, out: &Path, guid: &str) -> String {
    let t = &p.texts;
    let app = &p.app;
    let messages = match t.code {
        "pt" => "compiler:Languages\\BrazilianPortuguese.isl",
        "es" => "compiler:Languages\\Spanish.isl",
        _ => "compiler:Default.isl",
    };
    let exe_rel = format!("{{app}}\\{}{}", p.exe_sub(), p.exe_name);
    let exe_dir = format!("{{app}}\\{}", p.exe_sub()).trim_end_matches('\\').to_string();
    let autostart = p.offers(&p.w.autostart);
    let desktop = p.offers(&p.w.desktop_shortcut);
    let admin = p.offers(&p.w.run_as_admin);
    let choices = autostart.is_some() || desktop.is_some() || admin.is_some();
    let base = p.out_name.trim_end_matches(".exe");
    let mut s = String::new();
    let mut line = |text: &str| {
        s.push_str(text);
        s.push('\n');
    };
    line("; Gerado pelo Aylis (Publicar → Windows → instalador Inno Setup). Não edite: refeito a cada publicação.");
    line("[Setup]");
    // O AppId fica igual em todas as versões: a nova substitui a anterior.
    line(&format!("AppId={{{{{guid}}}"));
    line(&format!("AppName={app}"));
    line(&format!("AppVersion={}", p.version));
    line(&format!("AppPublisher={}", p.publisher));
    line(&format!("VersionInfoVersion={}", p.version4));
    line(&format!("VersionInfoProductName={app}"));
    line(&format!("VersionInfoCompany={}", p.publisher));
    line(&format!("VersionInfoDescription={} {app}", t.description));
    line(&format!("VersionInfoCopyright={}", p.publisher));
    line(&format!("DefaultDirName={}", p.install_dir(false)));
    line(&format!("DefaultGroupName={app}"));
    line("DisableProgramGroupPage=yes");
    line("PrivilegesRequired=lowest");
    line("ArchitecturesAllowed=x64compatible");
    line("ArchitecturesInstallIn64BitMode=x64compatible");
    line(&format!("OutputDir={}", out.display()));
    line(&format!("OutputBaseFilename={base}"));
    if let Some(icon) = &p.icon {
        line(&format!("SetupIconFile={}", icon.display()));
    }
    line(&format!("UninstallDisplayIcon={exe_rel}"));
    line(&format!("UninstallDisplayName={app}"));
    line("Compression=lzma2");
    line("SolidCompression=yes");
    line("WizardStyle=modern");
    // Com o app aberto, o Gerenciador de Reinicialização do Windows mostra e fecha.
    line(&format!("CloseApplications={}", if p.w.close_running { "yes" } else { "no" }));
    line("");
    line("[Languages]");
    line(&format!("Name: \"main\"; MessagesFile: \"{messages}\""));
    if choices {
        line("");
        line("[Tasks]");
        if let Some(on) = autostart {
            line(&format!("Name: \"autostart\"; Description: {}{}", inno_str(t.autostart), if on { "" } else { "; Flags: unchecked" }));
        }
        if let Some(on) = desktop {
            line(&format!("Name: \"desktopicon\"; Description: {}{}", inno_str(t.desktop), if on { "" } else { "; Flags: unchecked" }));
        }
        if let Some(on) = admin {
            line(&format!("Name: \"runasadmin\"; Description: {}{}", inno_str(t.run_as_admin), if on { "" } else { "; Flags: unchecked" }));
        }
    }
    line("");
    line("[Files]");
    line(&format!("Source: {}; DestDir: \"{exe_dir}\"; Flags: ignoreversion", inno_str(&p.exe.display().to_string())));
    for f in &p.extra {
        line(&format!("Source: {}; DestDir: \"{exe_dir}\"; Flags: ignoreversion", inno_str(&f.display().to_string())));
    }
    line("");
    line("[Icons]");
    line(&format!("Name: \"{{autoprograms}}\\{app}\"; Filename: \"{exe_rel}\""));
    if desktop.is_some() {
        line(&format!("Name: \"{{autodesktop}}\\{app}\"; Filename: \"{exe_rel}\"; Tasks: desktopicon"));
    }
    if autostart.is_some() || admin.is_some() || !p.schemes.is_empty() {
        line("");
        line("[Registry]");
    }
    if admin.is_some() {
        // Só a escolha, para o app ler (o Inno lembra a tarefa ao atualizar).
        let key = format!("Software\\{app}");
        line(&format!("Root: HKCU; Subkey: \"{key}\"; ValueType: dword; ValueName: \"RunAsAdmin\"; ValueData: 1; Tasks: runasadmin; Flags: uninsdeletevalue"));
        line(&format!("Root: HKCU; Subkey: \"{key}\"; ValueType: dword; ValueName: \"RunAsAdmin\"; ValueData: 0; Tasks: not runasadmin; Flags: uninsdeletevalue"));
    }
    for (scheme, description) in &p.schemes {
        let key = format!("Software\\Classes\\{scheme}");
        line(&format!("Root: HKCU; Subkey: \"{key}\"; ValueType: string; ValueName: \"\"; ValueData: {}; Flags: uninsdeletekey", inno_str(&format!("URL:{description}"))));
        line(&format!("Root: HKCU; Subkey: \"{key}\"; ValueType: string; ValueName: \"URL Protocol\"; ValueData: \"\""));
        line(&format!("Root: HKCU; Subkey: \"{key}\\DefaultIcon\"; ValueType: string; ValueName: \"\"; ValueData: \"\"\"{exe_rel}\"\",0\""));
        line(&format!("Root: HKCU; Subkey: \"{key}\\shell\\open\\command\"; ValueType: string; ValueName: \"\"; ValueData: \"\"\"{exe_rel}\"\" \"\"%1\"\"\""));
    }
    if autostart.is_some() {
        let args = if p.w.autostart_args.is_empty() { String::new() } else { format!(" {}", p.w.autostart_args) };
        let run = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
        line(&format!("Root: HKCU; Subkey: \"{run}\"; ValueType: string; ValueName: \"{app}\"; ValueData: \"\"\"{exe_rel}\"\"{args}\"; Tasks: autostart; Flags: uninsdeletevalue"));
        // Desmarcado: tira a entrada de uma instalação anterior.
        line(&format!("Root: HKCU; Subkey: \"{run}\"; ValueType: none; ValueName: \"{app}\"; Tasks: not autostart; Flags: deletevalue"));
    }
    if p.w.run_after_install {
        line("");
        line("[Run]");
        line(&format!("Filename: \"{exe_rel}\"; Flags: nowait postinstall skipifsilent"));
    }
    if !p.w.data_dir.is_empty() {
        line("");
        line("[Code]");
        line("procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);");
        line("begin");
        line("  if CurUninstallStep = usPostUninstall then");
        line(&format!("    if MsgBox('{}', mbConfirmation, MB_YESNO) = IDYES then", t.remove_data.replace('\'', "''")));
        line(&format!("      DelTree(ExpandConstant('{}'), True, True, True);", expand(&p.w.data_dir, false).replace('\'', "''")));
        line("end;");
    }
    s
}

/// O `makensis` do NSIS: o das ferramentas da IDE, o de `NSIS_HOME`, o instalado ou o do PATH.
pub fn makensis() -> Option<PathBuf> {
    let home = std::env::var_os("NSIS_HOME").map(|h| PathBuf::from(h).join("makensis.exe"));
    let installed = ["ProgramFiles(x86)", "ProgramFiles"].iter().filter_map(|v| std::env::var_os(v)).map(|d| PathBuf::from(d).join("NSIS").join("makensis.exe"));
    crate::settings::tool("makensis")
        .into_iter()
        .chain(home)
        .chain(installed)
        .find(|p| p.is_file())
        .or_else(|| crate::hostlang::which("makensis"))
}

/// O `ISCC` do Inno Setup: o das ferramentas da IDE, o instalado ou o do PATH.
pub fn iscc() -> Option<PathBuf> {
    let installed = ["ProgramFiles(x86)", "ProgramFiles", "LOCALAPPDATA"].iter().filter_map(|v| std::env::var_os(v)).flat_map(|d| {
        let d = PathBuf::from(d);
        [d.join("Inno Setup 6").join("ISCC.exe"), d.join("Programs").join("Inno Setup 6").join("ISCC.exe")]
    });
    crate::settings::tool("iscc").into_iter().chain(installed).find(|p| p.is_file()).or_else(|| crate::hostlang::which("iscc"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_has_four_numeric_parts() {
        assert_eq!(version4("0.52.7-beta"), "0.52.7.0");
        assert_eq!(version4("1.2"), "1.2.0.0");
        assert_eq!(version4("2.0.1.9"), "2.0.1.9");
    }

    /// Os prefixos dos `UrlScheme` ligados vão para os dois instaladores, que tiram na
    /// desinstalação; o desligado e o inválido ficam de fora.
    #[test]
    fn url_schemes_are_registered_and_removed_by_the_installers() {
        let base = std::env::temp_dir().join(format!("aylis_url_schemes_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
        std::fs::write(
            project.dir().join("forms").join("links.json"),
            r#"{ "root": { "class": "Form", "children": [
                { "class": "UrlScheme", "name": "a", "scheme": "Pedidos", "description": "Pedidos da loja", "active": true },
                { "class": "UrlScheme", "name": "b", "scheme": "desligado", "active": false },
                { "class": "UrlScheme", "name": "c", "scheme": "https", "active": true } ] } }"#,
        )
        .unwrap();
        assert_eq!(super::super::url_schemes(&project), [("pedidos".to_string(), "Pedidos da loja".to_string())]);
        let p = Plan::new(&project, Path::new("C:/build/loja.exe"), &[]);
        let nsis = nsis_script(&p, Path::new("C:/out/Loja-setup.exe"));
        let uninstall = &nsis[nsis.find("Section \"Uninstall\"").unwrap()..];
        assert!(nsis.contains(r#"WriteRegStr HKCU "Software\Classes\pedidos" "URL Protocol" """#), "{nsis}");
        assert!(nsis.contains(r#"WriteRegStr HKCU "Software\Classes\pedidos\shell\open\command" "" '""#) && nsis.contains(r#"loja.exe" "%1"'"#), "{nsis}");
        assert!(uninstall.contains(r#"DeleteRegKey HKCU "Software\Classes\pedidos""#), "{uninstall}");
        assert!(!nsis.contains("desligado") && !nsis.contains(r"Classes\https"));
        let inno = inno_script(&p, Path::new("C:/out"), "{0000}");
        assert!(inno.contains(r#"Subkey: "Software\Classes\pedidos"; ValueType: string; ValueName: ""; ValueData: "URL:Pedidos da loja"; Flags: uninsdeletekey"#), "{inno}");
        assert!(inno.contains(r#"loja.exe"" ""%1"""#), "{inno}");
        assert_eq!(inno.matches("[Registry]").count(), 1);
        let _ = std::fs::remove_dir_all(&base);
    }

    /// "Abrir como administrador": a caixa nos dois instaladores, que só gravam a escolha
    /// (1 ou 0) para o app; o NSIS a mantém ao atualizar e, com o app aberto que não fecha,
    /// avisa e para.
    #[test]
    fn run_as_admin_is_offered_and_saved_for_the_app() {
        let base = std::env::temp_dir().join(format!("aylis_run_as_admin_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let mut project = aylis_project::create(&base, "Loja", aylis_project::Language::Rust).unwrap();
        project.publish.windows.close_running = true;

        let p = Plan::new(&project, Path::new("C:/build/loja.exe"), &[]);
        let nsis = nsis_script(&p, Path::new("C:/out/Loja-setup.exe"));
        assert!(!nsis.contains("RunAsAdmin") && !nsis.contains("MUI_PAGE_COMPONENTS"), "sem a opção, nada: {nsis}");
        assert!(nsis.contains("taskkill") && nsis.contains("MB_ICONSTOP"), "o app que não fecha para a instalação: {nsis}");

        project.publish.windows.run_as_admin = "on".into();
        let p = Plan::new(&project, Path::new("C:/build/loja.exe"), &[]);
        let nsis = nsis_script(&p, Path::new("C:/out/Loja-setup.exe"));
        assert!(nsis.contains(&format!("Section \"{}\" SecRunAsAdmin", p.texts.run_as_admin)), "{nsis}");
        assert!(nsis.contains(r#"WriteRegDWORD HKCU "Software\Loja" "RunAsAdmin" 1"#), "{nsis}");
        assert!(nsis.contains(r#"WriteRegDWORD HKCU "Software\Loja" "RunAsAdmin" 0"#), "{nsis}");
        assert!(nsis.contains("MUI_PAGE_COMPONENTS") && nsis.contains(r#"ReadRegDWORD $1 HKCU "Software\Loja" "RunAsAdmin""#), "{nsis}");
        assert!(nsis.contains("RequestExecutionLevel user"), "o instalador continua sem pedir administrador");
        let inno = inno_script(&p, Path::new("C:/out"), "{0000}");
        assert!(inno.contains(&format!("Name: \"runasadmin\"; Description: \"{}\"\n", p.texts.run_as_admin)), "{inno}");
        assert!(inno.contains(r#"ValueName: "RunAsAdmin"; ValueData: 1; Tasks: runasadmin"#), "{inno}");
        assert!(inno.contains(r#"ValueName: "RunAsAdmin"; ValueData: 0; Tasks: not runasadmin"#), "{inno}");
        assert!(inno.contains("PrivilegesRequired=lowest"));

        project.publish.windows.run_as_admin = "off".into();
        let p = Plan::new(&project, Path::new("C:/build/loja.exe"), &[]);
        assert!(nsis_script(&p, Path::new("C:/out/Loja-setup.exe")).contains(&format!("Section /o \"{}\" SecRunAsAdmin", p.texts.run_as_admin)));
        assert!(inno_script(&p, Path::new("C:/out"), "{0000}").contains(&format!("Description: \"{}\"; Flags: unchecked", p.texts.run_as_admin)));
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn folder_marks_become_each_installer_variables() {
        assert_eq!(expand("{localappdata}/NoOBS", true), "$LOCALAPPDATA\\NoOBS");
        assert_eq!(expand("{localappdata}/NoOBS", false), "{localappdata}\\NoOBS");
        assert_eq!(expand("{programfiles}\\App", false), "{autopf}\\App");
    }
}
