//! O instalador da IDE para Windows (`aylis-tool installer-ide`): compila em release, gera
//! o script do NSIS e chama o `makensis`.
//!
//! Uma página de componentes, agrupada pelo que se programa e para onde se publica:
//!
//! ```text
//! Aylis IDE e Rust (obrigatório)   a IDE, o aylis-host, o aylis-tool, o código do Aylis (crates,
//!                                  third_party: os projetos em Rust compilam contra ele), os
//!                                  clientes das linguagens, a documentação; e o Rust configurado
//! Outras linguagens                Python, TypeScript, C#, Java, Go, C++, Free Pascal (opcionais)
//! Plataformas de destino           Android, Navegador (opcionais; o Windows já vem)
//! Atalho na área de trabalho
//! ```
//!
//! Sem o Rust o Aylis não funciona (a IDE, os componentes e a publicação passam por ele), e
//! por isso ele não é opção. Uma linguagem ou uma plataforma marcada vem com tudo o que precisa
//! para funcionar bem: no fim da instalação, o `aylis-tool` recém-instalado roda o
//! `aylis-tool setup` dela ([`crate::setup`], o mesmo do "Configurar linguagens" da IDE), que
//! instala o compilador ou o interpretador, o depurador e o servidor de linguagem (o Android: o
//! JDK, o SDK, o NDK e o cargo-ndk; o navegador: o alvo wasm e o wasm-bindgen). Tudo vem dos
//! pacotes de ferramentas (`docs/pacotes.md`), conferidos com o SHA-256, da pasta `pacotes` ao
//! lado do instalador (o kit offline) ou do espelho padrão
//! ([`crate::settings::DEFAULT_PACKAGE_MIRROR`]); o que não tem pacote vem pelo winget. A saída
//! fica nos detalhes da instalação (`nsExec`, sem `cmd`). O que não se configurar é dito no
//! fim, com o caminho para refazer (Ferramentas → Configurar linguagens), e a IDE fica.
//!
//! Os projetos de exemplo não vão (o repositório os guarda): o `Cargo.toml` instalado é o do
//! workspace sem eles entre os membros. As páginas de referência dos componentes (as que o
//! MCP da IDE lê) vão para `aylis\docs\referencia`.
//!
//! Instala por usuário (`%LOCALAPPDATA%\Programs\aylis`, sem pedir administrador; o Windows
//! ainda pede a permissão no que o winget instala para a máquina, como o Visual C++); as
//! compilações dos projetos ficam fora da instalação (`crate::shared_target_dir`). A IDE acha
//! a pasta `aylis\` sozinha, ao lado do executável (`aylis_project::codegen::aylis_root`).
//!
//! Em modo silencioso (ou para já abrir marcado):
//!
//! ```text
//! aylis-ide-<versão>-setup.exe /S /LINGUAGENS=python,go /PLATAFORMAS=android,web /ESPELHO=https://... /D=C:\pasta
//! ```
//!
//! `/LINGUAGENS=` e `/PLATAFORMAS=` aceitam os nomes separados por vírgula (sem espaços) ou
//! `all`; `/ESPELHO=` acrescenta um espelho aos das Opções. O código de saída é 3 quando a IDE
//! foi instalada e algo não se configurou. O desinstalador remove também os pacotes
//! (`%USERPROFILE%\.aylis\tools`), o cache deles (`downloads`) e o `environment.json`; as
//! Opções (`tools.json`) ficam.
//!
//! ## O kit offline
//!
//! `aylis-tool installer-ide --with-packages node,codelldb` (ou `all`, ou sem nomes: todos)
//! gera o setup e, ao lado dele, a pasta `pacotes` (o `packages pack` para Windows x86_64,
//! com as dependências). A pasta e o setup juntos (numa pasta de rede, num pendrive)
//! instalam sem internet.

use std::path::{Path, PathBuf};
use std::process::Command;

use aylis_project::Language;

use crate::env::exe;
use crate::toolpacks::Manifest;

/// A plataforma do instalador (os pacotes do kit offline).
const PLATFORM: &str = "windows-x86_64";

/// As plataformas de destino além do Windows: (o `--target` do `aylis-tool setup`, o nome, o
/// que vem junto).
const TARGETS: &[(&str, &str, &str)] = &[
    ("android", "Android", "Publicar e depurar no Android: o alvo do Rust, o JDK, o Android SDK, o NDK e o cargo-ndk."),
    ("web", "Navegador (web)", "Publicar para o navegador (wasm): o alvo do Rust e o wasm-bindgen."),
];

/// As linguagens opcionais: todas menos o Rust (que sempre vem).
fn languages() -> impl Iterator<Item = Language> {
    Language::ALL.into_iter().filter(|l| *l != Language::Rust)
}

/// Os executáveis (o pacote do cargo e o nome do binário).
const BINARIES: &[(&str, &str)] = &[("aylis-ide", "aylis-ide"), ("aylis-host", "aylis-host"), ("aylis-tools", "aylis-tool"), ("aylis-migrate", "aylis-migrate")];

/// O `Cargo.toml` do workspace sem os exemplos entre os membros (os exemplos não vão para o
/// instalador; com eles listados, o cargo recusaria o workspace instalado).
fn without_examples(manifest: &str) -> String {
    manifest
        .lines()
        .map(|line| {
            let key = line.trim_start();
            if !(key.starts_with("members") || key.starts_with("exclude")) {
                return line.to_string();
            }
            let Some((head, list)) = line.split_once('[') else { return line.to_string() };
            let list = list.trim_end().trim_end_matches(']');
            let kept: Vec<&str> = list.split(',').map(str::trim).filter(|m| !m.is_empty() && !m.trim_matches('"').starts_with("examples/")).collect();
            format!("{head}[{}]", kept.join(", "))
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// Saídas de compilação e coisas locais que não vão para o instalador (nomes, em qualquer
/// nível das pastas copiadas).
/// `.aylis` (a compilação local do projeto), `.cache` (o índice do clangd), `dist` (o
/// publicado) e o `ibge-cache` (as respostas da API) são da máquina de quem gera.
const SKIP: &[&str] = &[
    "target",
    "pkg",
    "node_modules",
    "bin",
    "obj",
    "out",
    "__pycache__",
    ".claude",
    ".aylis",
    ".cache",
    "dist",
    "ibge-cache",
    "compile_commands.json",
    "*.pyc",
    "*.exe",
    "*.obj",
    "*.pdb",
];

pub struct Options {
    /// Pasta onde o instalador é gravado (padrão: `dist/` no repositório).
    pub out: Option<PathBuf>,
    /// Usa os binários já compilados em `target/release`.
    pub skip_build: bool,
    /// O kit offline: grava também `<out>\pacotes` com estes pacotes (vazio: todos).
    pub with_packages: Option<Vec<String>>,
}

/// Gera o instalador; devolve o caminho do `.exe`.
/// A auditoria das dependências (`cargo deny check`, regras em `deny.toml`): falhas de
/// segurança conhecidas (RustSec), licenças e de onde vêm os crates. Erro se algo falhar.
pub fn audit(root: &Path) -> Result<(), String> {
    println!("▶ auditoria das dependências (cargo deny)");
    let status = Command::new("cargo")
        .args(["deny", "--log-level", "error", "check", "advisories", "licenses", "sources", "bans"])
        .current_dir(root)
        .status()
        .map_err(|e| format!("cargo: {e}"))?;
    match status.code() {
        Some(0) => Ok(()),
        // O cargo sem o subcomando sai com 101 e diz "no such command".
        Some(101) => Err("a auditoria precisa do cargo-deny: cargo install cargo-deny --locked".into()),
        _ => Err("a auditoria das dependências falhou (veja acima; regras em deny.toml)".into()),
    }
}

pub fn installer_ide(options: &Options) -> Result<PathBuf, String> {
    if !cfg!(windows) {
        return Err("o instalador da IDE é para Windows (gere numa máquina Windows)".into());
    }
    let makensis = makensis().ok_or("makensis não encontrado: instale o NSIS (https://nsis.sourceforge.io) ou informe o caminho nas Opções da IDE")?;
    // Um nome errado no kit aparece agora, não depois da compilação.
    if let Some(names) = &options.with_packages {
        Manifest::embedded().with_dependencies(&names.iter().map(String::as_str).collect::<Vec<_>>())?;
    }
    let root = crate::aylis_root();
    if !root.join("crates").is_dir() {
        return Err(format!("{} não é o repositório do Aylis (rode o installer-ide de dentro dele)", root.display()));
    }
    // As dependências passam pela auditoria antes (falha de segurança conhecida, licença).
    audit(&root)?;
    let target = crate::cargo_target_dir(&root);
    let release = target.join("release");
    if !options.skip_build {
        println!("▶ compilando em release: aylis-ide, aylis-host, aylis-tool");
        let mut cmd = Command::new("cargo");
        cmd.args(["build", "--release"]).current_dir(&root);
        for (package, _) in BINARIES {
            cmd.args(["-p", package]);
        }
        if !cmd.status().map_err(|e| format!("cargo: {e}"))?.success() {
            return Err("a compilação falhou".into());
        }
    }
    let missing: Vec<_> = BINARIES.iter().map(|(_, b)| exe(b)).filter(|b| !release.join(b).is_file()).collect();
    if !missing.is_empty() {
        return Err(format!("não achei em {}: {}", release.display(), missing.join(", ")));
    }

    let out_dir = options.out.clone().unwrap_or_else(|| root.join("dist"));
    std::fs::create_dir_all(&out_dir).map_err(|e| format!("{}: {e}", out_dir.display()))?;
    let version = env!("CARGO_PKG_VERSION");
    let setup = out_dir.join(format!("aylis-ide-{version}-setup.exe"));

    // O script e o LEIA-ME ficam no target (são gerados a cada vez).
    let work = target.join("installer-ide");
    std::fs::create_dir_all(&work).map_err(|e| format!("{}: {e}", work.display()))?;
    let readme = work.join("LEIA-ME.txt");
    std::fs::write(&readme, README).map_err(|e| e.to_string())?;
    // Um ícone por tipo de arquivo do aylis (o logo da IDE numa folha, com a marca do tipo).
    let icons = work.join("icons");
    std::fs::create_dir_all(&icons).map_err(|e| format!("{}: {e}", icons.display()))?;
    let ide_icon = root.join("crates").join("aylis-ide").join("resources").join("aylis-ide.ico");
    let logo = crate::icons::from_ico(&std::fs::read(&ide_icon).map_err(|e| format!("{}: {e}", ide_icon.display()))?)?;
    for (ext, _, _) in crate::icons::FILE_TYPES {
        let ico = crate::icons::file_type_ico(&logo, ext).expect("tipo do Aylis");
        std::fs::write(icons.join(format!("{ext}.ico")), ico).map_err(|e| e.to_string())?;
    }
    let manifest = work.join("Cargo.toml");
    let text = std::fs::read_to_string(root.join("Cargo.toml")).map_err(|e| e.to_string())?;
    std::fs::write(&manifest, without_examples(&text)).map_err(|e| format!("{}: {e}", manifest.display()))?;
    let nsi = work.join("aylis-ide.nsi");
    // Com BOM: o NSIS lê o script como UTF-8 (os acentos dos textos).
    let script = format!("\u{feff}{}", script(&root, &release, &readme, &icons, &manifest, &setup, version));
    std::fs::write(&nsi, script).map_err(|e| format!("{}: {e}", nsi.display()))?;

    println!("▶ gerando o instalador (makensis)");
    let status = Command::new(&makensis).args(["/V2", "/INPUTCHARSET", "UTF8"]).arg(&nsi).status().map_err(|e| format!("makensis: {e}"))?;
    if !status.success() {
        return Err(format!("o makensis falhou (script em {})", nsi.display()));
    }
    if let Some(names) = &options.with_packages {
        offline_kit(&out_dir.join("pacotes"), names)?;
    }
    Ok(setup)
}

/// O kit offline: os arquivos dos pacotes `names` (vazio: todos) e das dependências para
/// Windows x86_64 em `out`, com o `pacotes.json` (o `aylis-tool packages pack`). O instalador
/// os acha na pasta `pacotes` ao lado dele.
fn offline_kit(out: &Path, names: &[String]) -> Result<(), String> {
    use crate::toolpacks::{Paths, Progress, pack};
    let what = if names.is_empty() { "todos".to_string() } else { names.join(", ") };
    println!("▶ kit offline ({what}) em {}", out.display());
    let paths = Paths::user().ok_or("sem a pasta do usuário (USERPROFILE)")?;
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    let mut shown = std::collections::HashMap::<String, u64>::new();
    let report = pack(&Manifest::embedded(), &paths.cache, out, &names, &[PLATFORM.to_string()], &crate::process::Control::default(), &mut |p| match p {
        Progress::Step(s) => println!("  {s}"),
        Progress::Bytes { file, done, total, .. } if total > 0 => {
            let percent = done * 100 / total;
            let last = shown.entry(file.clone()).or_insert(0);
            if percent >= *last + 25 {
                *last = percent;
                println!("    {file}: {percent}%");
            }
        }
        Progress::Bytes { .. } => {}
    })?;
    println!("  {} arquivos, {} MB", report.files.len(), report.total().div_ceil(1 << 20));
    Ok(())
}

/// Um texto entre aspas duplas no script do NSIS (`$` e `"` escapados, numa linha só).
fn nsis_text(s: &str) -> String {
    s.replace('$', "$$").replace('"', "$\\\"").replace(['\r', '\n'], " ")
}

/// As seções das linguagens e das plataformas, as descrições delas e o marcar pela linha de
/// comando (`/LINGUAGENS=`, `/PLATAFORMAS=`).
struct ChoiceScript {
    languages: String,
    targets: String,
    descriptions: String,
    from_parameters: String,
}

/// Uma seção opcional que roda o `aylis-tool setup` (`args`) no fim; `select` marca pelo nome
/// na lista de `$R1` (`,nome,` ou `,all,`).
fn setup_section(id: &str, label: &str, args: &str, name: &str, description: &str, out: &mut ChoiceScript, group: fn(&mut ChoiceScript) -> &mut String) {
    group(out).push_str(&format!("Section /o \"{label}\" {id}\n  !insertmacro AylisSetup \"{args}\" \"{label}\"\nSectionEnd\n", label = nsis_text(label)));
    out.descriptions.push_str(&format!("  !insertmacro MUI_DESCRIPTION_TEXT ${{{id}}} \"{}\"\n", nsis_text(description)));
    out.from_parameters.push_str(&format!(
        "  ${{StrStr}} $R2 \"$R1\" \",{name},\"\n  StrCmp $R2 \"\" 0 aylis_on_{id}\n  ${{StrStr}} $R2 \"$R1\" \",all,\"\n  StrCmp $R2 \"\" aylis_next_{id}\n  aylis_on_{id}:\n  !insertmacro SelectSection ${{{id}}}\n  aylis_next_{id}:\n"
    ));
}

fn choice_script() -> ChoiceScript {
    let mut out = ChoiceScript { languages: String::new(), targets: String::new(), descriptions: String::new(), from_parameters: String::new() };
    out.from_parameters.push_str("  ClearErrors\n  ${GetOptions} $R0 \"/LINGUAGENS=\" $R1\n  StrCpy $R1 \",$R1,\"\n");
    for language in languages() {
        let folder = language.folder();
        // O nome vai para a linha de comando: só os do Aylis.
        assert!(aylis_project::is_identifier(folder), "{folder}");
        let description = format!("Programar em {}: o compilador ou o interpretador, o depurador e o servidor de linguagem, instalados e configurados.", language.name());
        setup_section(&format!("SecLang_{folder}"), language.name(), folder, folder, &description, &mut out, |o| &mut o.languages);
    }
    out.from_parameters.push_str("  ClearErrors\n  ${GetOptions} $R0 \"/PLATAFORMAS=\" $R1\n  StrCpy $R1 \",$R1,\"\n");
    for (id, name, description) in TARGETS {
        setup_section(&format!("SecTarget_{id}"), name, &format!("rust --target {id}"), id, description, &mut out, |o| &mut o.targets);
    }
    out
}

/// O `makensis`: o escolhido nas Opções da IDE, o `PATH` ou a pasta padrão do NSIS.
pub fn makensis() -> Option<PathBuf> {
    crate::settings::tool("makensis")
        .filter(|p| p.is_file())
        .or_else(|| crate::which("makensis"))
        .or_else(|| {
            ["ProgramFiles(x86)", "ProgramFiles"]
                .iter()
                .filter_map(std::env::var_os)
                .map(|p| PathBuf::from(p).join("NSIS").join("makensis.exe"))
                .find(|p| p.is_file())
        })
}

fn script(root: &Path, release: &Path, readme: &Path, icons: &Path, manifest: &Path, setup: &Path, version: &str) -> String {
    let p = |path: &Path| path.display().to_string().replace('/', "\\");
    let skip: String = SKIP.iter().map(|s| format!("/x {s} ")).collect();
    let tree = |name: &str| format!("  File /r {skip}\"{}\"\n", p(&root.join(name)));
    let file = |name: &str| format!("  File \"{}\"\n", p(&root.join(name)));
    let bins: String = BINARIES.iter().map(|(_, b)| format!("  File \"{}\"\n", p(&release.join(exe(b))))).collect();
    // O código do Aylis (os projetos em Rust compilam contra ele), sem os exemplos: o
    // `Cargo.toml` é a cópia sem eles entre os membros. O `third_party` também: o cargo exige
    // as dependências por caminho mesmo de outro alvo (a cópia do `accesskit_android`).
    let code = format!("  File \"{}\"\n{}{}{}{}", p(manifest), file("Cargo.lock"), tree("crates"), tree("third_party"), tree("clients"));
    // A documentação e, em `docs\referencia`, as páginas dos componentes (o MCP da IDE lê),
    // sem as imagens.
    let reference = root.join("examples").join("Landing").join("assets").join("docs").join("componentes");
    let docs = format!(
        "{}{}{}  SetOutPath \"$INSTDIR\\aylis\\docs\\referencia\"\n  File \"{}\\*.md\"\n  SetOutPath \"$INSTDIR\\aylis\"\n",
        tree("docs"),
        tree("schema"),
        file("README.md"),
        p(&reference)
    );
    let file_icons: String = crate::icons::FILE_TYPES.iter().map(|(ext, _, _)| format!("  File \"{}\"\n", p(&icons.join(format!("{ext}.ico"))))).collect();
    let choices = choice_script();
    TEMPLATE
        .replace("@LANGUAGE_SECTIONS@", &choices.languages)
        .replace("@TARGET_SECTIONS@", &choices.targets)
        .replace("@CHOICE_DESCRIPTIONS@", &choices.descriptions)
        .replace("@CHOICE_PARAMETERS@", &choices.from_parameters)
        .replace("@VERSION@", version)
        .replace("@ICON@", &p(&root.join("crates").join("aylis-ide").join("resources").join("aylis-ide.ico")))
        .replace("@SETUP@", &p(setup))
        .replace("@BINS@", &bins)
        // O desinstalar apaga os mesmos executáveis que o instalar copia.
        .replace("@UNBINS@", &BINARIES.iter().map(|(_, b)| format!("  Delete \"$INSTDIR\\{}\"\n", exe(b))).collect::<String>())
        .replace("@README@", &format!("  File \"{}\"\n{}", p(readme), file("LICENSE")))
        .replace("@LICENSE@", &p(&root.join("LICENSE")))
        .replace("@RESOURCES@", &tree("resources"))
        .replace("@FILEICONS@", &file_icons)
        .replace("@CODE@", &code)
        .replace("@DOCS@", &docs)
}

/// O script do NSIS (Modern UI, em português); os `@...@` são trocados em `script`.
const TEMPLATE: &str = r#"Unicode true
!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "WinMessages.nsh"
!include "StrFunc.nsh"
!include "Sections.nsh"
; As funções de texto usadas (o StrFunc pede a declaração antes).
${StrStr}
${UnStrRep}

!define UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\aylis-ide"

; O espelho a mais (/ESPELHO=) e o que não se configurou.
Var Mirror
Var SetupFailed

; Configura uma linguagem ou uma plataforma: o aylis-tool recém-instalado roda o setup dela
; (o compilador, o depurador, o servidor de linguagem), dos pacotes da pasta ao lado do
; instalador, das origens das Opções ou do espelho padrão. A saída vai para os detalhes (em
; UTF-16: o nsExec lê UTF-8 como ANSI). Falhar não desfaz a IDE.
!macro AylisSetup ARGS TITLE
  SetDetailsPrint both
  DetailPrint "Configurando ${TITLE}…"
  nsExec::ExecToLog '"$INSTDIR\aylis-tool.exe" setup ${ARGS} --utf16-output'
  Pop $R0
  StrCmp $R0 "0" +2
  StrCpy $SetupFailed "$SetupFailed, ${TITLE}"
!macroend

; Um tipo de arquivo do aylis (em HKCU\Software\Classes): a extensão, o nome, o ícone e o abrir.
!macro AylisFileType EXT PROGID DESC
  WriteRegStr HKCU "Software\Classes\.${EXT}" "" "${PROGID}"
  WriteRegStr HKCU "Software\Classes\${PROGID}" "" "${DESC}"
  WriteRegStr HKCU "Software\Classes\${PROGID}\DefaultIcon" "" "$INSTDIR\icons\${EXT}.ico"
  WriteRegStr HKCU "Software\Classes\${PROGID}\shell\open\command" "" '"$INSTDIR\aylis-ide.exe" "%1"'
!macroend

; Desfaz o tipo (a extensão só sai se ainda aponta para o aylis).
!macro AylisFileTypeRemove EXT PROGID
  ReadRegStr $0 HKCU "Software\Classes\.${EXT}" ""
  StrCmp $0 "${PROGID}" 0 +2
  DeleteRegKey HKCU "Software\Classes\.${EXT}"
  DeleteRegKey HKCU "Software\Classes\${PROGID}"
!macroend

Name "Aylis IDE @VERSION@"
OutFile "@SETUP@"
InstallDir "$LOCALAPPDATA\Programs\aylis"
InstallDirRegKey HKCU "${UNINST_KEY}" "InstallLocation"
RequestExecutionLevel user
SetCompressor /SOLID lzma
ShowInstDetails show

!define MUI_ABORTWARNING
!define MUI_ICON "@ICON@"
!define MUI_UNICON "@ICON@"
!define MUI_COMPONENTSPAGE_SMALLDESC
!define MUI_FINISHPAGE_RUN "$INSTDIR\aylis-ide.exe"
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "@LICENSE@"
!define MUI_COMPONENTSPAGE_TEXT_TOP "O Aylis e o Rust sempre se instalam. Marque as outras linguagens e as plataformas que vai usar: cada uma vem com tudo o que precisa (compilador, depurador, servidor de linguagem). O resto se configura depois, na IDE (Ferramentas → Configurar linguagens)."
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "PortugueseBR"

Section "Aylis IDE e Rust (obrigatório)" SecIde
  SectionIn RO
  ; Os milhares de arquivos sem a linha de cada um na lista de detalhes: escrever na lista é o
  ; que mais custa (medido em 12/11: 20 mil arquivos apagados em 28,3 s com a lista e 3,1 s
  ; sem ela). A barra mostra só a etapa.
  SetDetailsPrint textonly
  DetailPrint "Removendo a versão anterior…"
  ; Uma versão anterior sai inteira (sem sobrar arquivos que a nova não tem, como os
  ; exemplos das versões que os levavam).
  RMDir /r "$INSTDIR\aylis"
  DetailPrint "Copiando a IDE…"
  SetOutPath "$INSTDIR"
@BINS@@README@
  SetOutPath "$INSTDIR\aylis"
@RESOURCES@
  DetailPrint "Copiando o código do Aylis…"
@CODE@
  DetailPrint "Copiando a documentação…"
@DOCS@
  ; Os ícones dos tipos de arquivo (um por extensão).
  RMDir /r "$INSTDIR\icons"
  SetOutPath "$INSTDIR\icons"
@FILEICONS@
  ; Os atalhos abaixo começam na pasta de saída: a mesma de antes.
  SetOutPath "$INSTDIR\aylis"
  WriteUninstaller "$INSTDIR\desinstalar.exe"
  CreateDirectory "$SMPROGRAMS\Aylis"
  CreateShortcut "$SMPROGRAMS\Aylis\Aylis IDE.lnk" "$INSTDIR\aylis-ide.exe"
  CreateShortcut "$SMPROGRAMS\Aylis\Desinstalar o Aylis.lnk" "$INSTDIR\desinstalar.exe"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayName" "Aylis IDE"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayVersion" "@VERSION@"
  WriteRegStr HKCU "${UNINST_KEY}" "Publisher" "aylis"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayIcon" "$INSTDIR\aylis-ide.exe"
  WriteRegStr HKCU "${UNINST_KEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "${UNINST_KEY}" "UninstallString" '"$INSTDIR\desinstalar.exe"'
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoModify" 1
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoRepair" 1
  ; Onde o aylis está, como qualquer app (as ferramentas e os scripts dos projetos acham por aqui):
  ; `SdkDir` é a pasta com os crates (a junção `.aylis\sdk` dos projetos aponta para ela).
  WriteRegStr HKCU "Software\Aylis" "InstallDir" "$INSTDIR"
  WriteRegStr HKCU "Software\Aylis" "SdkDir" "$INSTDIR\aylis"
  WriteRegStr HKCU "Software\Aylis" "Version" "@VERSION@"
  ; Os arquivos do aylis abrem com dois cliques (só deste usuário, sem administrador): o projeto,
  ; o pacote de componentes (instala), a gravação de uso e o relato de problema (os visualizadores).
  !insertmacro AylisFileType "aylisproj" "aylis.Projeto" "Projeto do Aylis"
  !insertmacro AylisFileType "aylispkg" "aylis.Pacote" "Pacote de componentes do Aylis"
  !insertmacro AylisFileType "aylisrec" "aylis.Gravacao" "Gravação de uso da IDE do Aylis"
  !insertmacro AylisFileType "aylisreport" "aylis.Relato" "Relato de problema do Aylis"
  System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, p 0, p 0)'
  ; A pasta no PATH do usuário: os terminais chamam o `aylis-tool` (e o `aylis-tool mcp`)
  ; pelo nome. Só se ainda não estiver.
  ReadRegStr $0 HKCU "Environment" "Path"
  ${StrStr} $1 "$0" "$INSTDIR"
  StrCmp $1 "" 0 aylis_path_done
  StrCmp $0 "" 0 +3
  StrCpy $0 "$INSTDIR"
  Goto +2
  StrCpy $0 "$0;$INSTDIR"
  WriteRegExpandStr HKCU "Environment" "Path" "$0"
  SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
  aylis_path_done:
SectionEnd

; O espelho de /ESPELHO= vai para as Opções (pelo aylis-tool, como a IDE grava), antes das
; configurações. A pasta `pacotes` ao lado do instalador (o kit offline) vale só para elas,
; pela variável (não fica gravada).
Section "-Origens"
  IfFileExists "$EXEDIR\pacotes\*.*" 0 +2
  System::Call 'kernel32::SetEnvironmentVariable(t "AYLIS_PACKAGE_DIR", t "$EXEDIR\pacotes")'
  StrCmp $Mirror "" aylis_mirror_done
  Call AylisCheckMirror
  StrCmp $R0 "" 0 aylis_mirror_bad
  SetDetailsPrint both
  DetailPrint "Gravando o espelho dos pacotes: $Mirror"
  nsExec::Exec '"$INSTDIR\aylis-tool.exe" packages sources --add-mirror "$Mirror"'
  Pop $R1
  StrCmp $R1 "0" aylis_mirror_done
  DetailPrint "Não deu para gravar o espelho nas Opções ($R1)."
  Goto aylis_mirror_done
  aylis_mirror_bad:
  SetDetailsPrint both
  DetailPrint "Espelho ignorado: $R0"
  aylis_mirror_done:
SectionEnd

; O Rust sempre: sem ele o Aylis não compila nada (o compilador, o Visual C++, o depurador e
; o servidor de linguagem).
Section "-Rust"
  !insertmacro AylisSetup "rust" "Rust"
SectionEnd

SectionGroup /e "Outras linguagens" SecLanguages
@LANGUAGE_SECTIONS@SectionGroupEnd

SectionGroup /e "Plataformas de destino" SecTargets
@TARGET_SECTIONS@SectionGroupEnd

Section /o "Atalho na área de trabalho" SecDesktop
  CreateShortcut "$DESKTOP\Aylis IDE.lnk" "$INSTDIR\aylis-ide.exe"
SectionEnd

; O tamanho em "Aplicativos instalados" (depois de tudo copiado).
Section "-Tamanho"
  ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
  WriteRegDWORD HKCU "${UNINST_KEY}" "EstimatedSize" $0
SectionEnd

; O que não se configurou: a IDE fica, e a linguagem se configura depois pela IDE.
Section "-O que não se configurou"
  StrCmp $SetupFailed "" aylis_setup_ok
  StrCpy $R0 $SetupFailed "" 2
  SetDetailsPrint both
  DetailPrint "Não ficaram prontas: $R0. Configure-as depois na IDE, em Ferramentas → Configurar linguagens."
  MessageBox MB_ICONEXCLAMATION|MB_OK "Não deu para deixar prontas: $R0.$\r$\n$\r$\nA IDE foi instalada. O motivo está nos detalhes da instalação (sem internet, um pacote que não confere com o SHA-256 do Aylis ou uma instalação recusada).$\r$\n$\r$\nConfigure depois na IDE, em Ferramentas → Configurar linguagens (ou com aylis-tool setup <linguagem>)." /SD IDOK
  SetErrorLevel 3
  aylis_setup_ok:
SectionEnd

; O espelho em $Mirror confere? $R0: vazio, ou o motivo.
Function AylisCheckMirror
  StrCpy $R0 ""
  StrCmp $Mirror "" aylis_check_done
  StrCpy $R1 $Mirror 8
  StrCmp $R1 "https://" 0 aylis_check_bad
  ${StrStr} $R1 "$Mirror" " "
  StrCmp $R1 "" 0 aylis_check_bad
  ${StrStr} $R1 "$Mirror" '"'
  StrCmp $R1 "" aylis_check_done aylis_check_bad
  aylis_check_bad:
  StrCpy $R0 "o espelho tem de ser um endereço HTTPS (https://...), sem espaços nem aspas"
  aylis_check_done:
FunctionEnd

; /LINGUAGENS=python,go, /PLATAFORMAS=android,web (ou all) e /ESPELHO=https://... na linha
; de comando (o modo silencioso, /S, usa só eles). Sem eles, só o Rust.
Function .onInit
  ${GetParameters} $R0
  ; O espelho à mão: o GetOptions corta o valor no primeiro "/" (o "https:" da URL). Vai do
  ; "/ESPELHO=" até a próxima opção (" /") ou o fim, sem as aspas.
  StrCpy $Mirror ""
  ${StrStr} $R1 "$R0" "/ESPELHO="
  StrCmp $R1 "" aylis_mirror_parsed
  StrCpy $R1 $R1 "" 9
  ${StrStr} $R2 "$R1" " /"
  StrLen $R3 $R2
  StrLen $R4 $R1
  IntOp $R4 $R4 - $R3
  StrCpy $Mirror $R1 $R4
  StrCpy $R2 $Mirror 1
  StrCmp $R2 '"' 0 +2
  StrCpy $Mirror $Mirror "" 1
  StrCpy $R2 $Mirror 1 -1
  StrCmp $R2 '"' 0 +2
  StrCpy $Mirror $Mirror -1
  StrCpy $R2 $Mirror 1 -1
  StrCmp $R2 " " 0 +2
  StrCpy $Mirror $Mirror -1
  aylis_mirror_parsed:
@CHOICE_PARAMETERS@FunctionEnd

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecIde} "A IDE, o código do Aylis, a documentação e o Rust (o compilador, o Visual C++, o depurador e o servidor de linguagem): sem ele o Aylis não funciona."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecLanguages} "Programar os eventos em outras linguagens: cada uma marcada vem pronta para executar e depurar."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecTargets} "Publicar para outras plataformas além do Windows."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDesktop} "Um atalho da IDE na área de trabalho."
@CHOICE_DESCRIPTIONS@!insertmacro MUI_FUNCTION_DESCRIPTION_END

Section "Uninstall"
  ; Sem a linha de cada arquivo na lista (veja a seção IDE): com a pasta de compilação são
  ; dezenas de milhares, e a lista levava minutos.
  SetDetailsPrint textonly
  DetailPrint "Removendo o Aylis…"
  Delete "$DESKTOP\Aylis IDE.lnk"
  RMDir /r "$SMPROGRAMS\Aylis"
  RMDir /r "$INSTDIR\aylis"
  RMDir /r "$INSTDIR\icons"
@UNBINS@  Delete "$INSTDIR\LEIA-ME.txt"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\desinstalar.exe"
  RMDir "$INSTDIR"
  ; As compilações dos projetos (cache; refeitas sob demanda).
  DetailPrint "Removendo as compilações dos projetos…"
  RMDir /r "$PROFILE\.aylis\target"
  ; Os pacotes de ferramentas que o Aylis instalou e o cache deles (o que a máquina já tinha
  ; fica: um pacote nunca vai para fora daqui), e a verificação de ambiente guardada. As
  ; Opções (tools.json) ficam, como numa reinstalação.
  DetailPrint "Removendo os pacotes de ferramentas do Aylis…"
  RMDir /r "$PROFILE\.aylis\tools"
  RMDir /r "$PROFILE\.aylis\downloads"
  Delete "$PROFILE\.aylis\environment.json"
  RMDir "$PROFILE\.aylis"
  DeleteRegKey HKCU "${UNINST_KEY}"
  DeleteRegKey HKCU "Software\Aylis"
  !insertmacro AylisFileTypeRemove "aylisproj" "aylis.Projeto"
  !insertmacro AylisFileTypeRemove "aylispkg" "aylis.Pacote"
  !insertmacro AylisFileTypeRemove "aylisrec" "aylis.Gravacao"
  !insertmacro AylisFileTypeRemove "aylisreport" "aylis.Relato"
  ; A pasta sai do PATH do usuário.
  ReadRegStr $0 HKCU "Environment" "Path"
  ${UnStrRep} $0 "$0" ";$INSTDIR" ""
  ${UnStrRep} $0 "$0" "$INSTDIR;" ""
  ${UnStrRep} $0 "$0" "$INSTDIR" ""
  WriteRegExpandStr HKCU "Environment" "Path" "$0"
  SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
  System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, p 0, p 0)'
SectionEnd
"#;

const README: &str = "Aylis IDE

  aylis-ide       a IDE
  aylis-host      roda os projetos em Python e nas outras linguagens (TypeScript, C#, Java, Go, C++, Free Pascal)
  aylis-tool      Executar/Publicar na linha de comando (aylis-tool check | run | publish | setup)
  aylis\\          o código do Aylis (os projetos em Rust compilam contra ele), os clientes das
                  linguagens, a documentação, as fontes e os ícones

O Rust sempre se instala: sem ele o Aylis não funciona. As outras linguagens e as plataformas
(Android, navegador) marcadas no instalador vêm prontas (o compilador ou o interpretador, o
depurador e o servidor de linguagem); as que faltarem se configuram depois na IDE
(Ferramentas -> Configurar linguagens) ou com: aylis-tool setup <linguagem>

As compilações ficam em %USERPROFILE%\\.aylis\\target (outra pasta nas Opções da IDE),
compartilhadas entre os projetos; o que nenhum projeto usa mais sai sozinho (ou com
aylis-tool clean).

Pacotes de ferramentas
  O que se instala vem dos pacotes do Aylis (em %USERPROFILE%\\.aylis\\tools): da pasta
  \"pacotes\" ao lado do instalador, das origens das Opções da IDE (aba Pacotes) ou do espelho
  padrão. Cada arquivo é conferido com o SHA-256 que o Aylis traz. O que não tem pacote vem
  pelo winget.

  Sem janelas (modo silencioso):
    aylis-ide-<versão>-setup.exe /S /LINGUAGENS=python,go /PLATAFORMAS=android,web /D=C:\\pasta
  Os nomes separados por vírgula, sem espaços, ou all (linguagens: python, typescript, csharp,
  java, go, cpp, pascal; plataformas: android, web). /ESPELHO=https://... acrescenta um
  espelho. Sai com o código 3 se a IDE foi instalada e algo não ficou pronto.

  O kit offline (para distribuir tudo junto, numa pasta de rede ou num pendrive):
    aylis-tool installer-ide --with-packages rust,jdk      (ou all)
  grava o setup e, ao lado dele, a pasta \"pacotes\" com os arquivos e as dependências. Copie
  os dois juntos; quem roda o setup instala sem internet.

  O desinstalador remove também os pacotes do Aylis e o cache deles
  (%USERPROFILE%\\.aylis\\tools e downloads); o que a máquina já tinha não é tocado.
";

#[cfg(test)]
mod tests {
    use super::*;

    fn generated_script() -> String {
        let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let x = Path::new("x");
        script(root, x, x, x, x, x, "0.0.0")
    }

    /// Nenhum exemplo vai: o workspace instalado não os lista (o cargo recusaria), e o script
    /// não copia a pasta `examples` (só as páginas de referência dos componentes, em
    /// `docs\referencia`).
    #[test]
    fn the_installer_leaves_the_examples_out() {
        let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.toml")).unwrap();
        assert!(manifest.contains("examples/Calculadora/rust"), "o repositório tem os exemplos");
        let installed = without_examples(&manifest);
        assert!(!installed.contains("examples/"), "{installed}");
        assert!(installed.contains("\"crates/aylis-core\"") && installed.contains("\"crates/aylis\""));
        assert!(!installed.contains(", ,") && !installed.contains("[, ") && !installed.contains(", ]"), "a lista continua válida");
        assert!(installed.parse::<toml_edit::DocumentMut>().is_ok(), "{installed}");
        let text = generated_script();
        let copies: Vec<&str> = text.lines().filter(|l| l.trim_start().starts_with("File")).collect();
        assert!(copies.iter().all(|l| !l.contains("\\examples\\") || l.contains("\\componentes\\*.md")), "{copies:#?}");
        assert!(text.contains("SetOutPath \"$INSTDIR\\aylis\\docs\\referencia\""));
        assert!(!text.contains("SetOutPath \"$INSTDIR\\aylis\\examples\""));
    }

    /// Uma página só: a IDE com o Rust (obrigatória), as outras linguagens e as plataformas
    /// em grupos, desmarcadas, cada uma rodando o `aylis-tool setup` dela; o Rust sempre roda.
    #[test]
    fn the_page_groups_the_languages_and_the_platforms() {
        let text = generated_script();
        assert!(!text.contains('@') || !text.lines().any(|l| l.contains("@LANGUAGE") || l.contains("@TARGET") || l.contains("@CHOICE") || l.contains("@CODE@")), "marcador sem trocar");
        assert_eq!(text.matches("!insertmacro MUI_PAGE_COMPONENTS").count(), 1, "uma página de componentes");
        assert!(!text.contains("Page custom"), "sem a página do espelho");
        let ide = &text[text.find("Section \"Aylis IDE e Rust (obrigatório)\" SecIde").expect("a IDE com o Rust")..];
        assert!(ide[..ide.find("SectionEnd").unwrap()].contains("SectionIn RO"));
        assert!(text.contains("Section \"-Rust\"\n  !insertmacro AylisSetup \"rust\" \"Rust\""), "o Rust sempre");
        let group = &text[text.find("SectionGroup /e \"Outras linguagens\"").unwrap()..];
        let group = &group[..group.find("SectionGroupEnd").unwrap()];
        for language in languages() {
            assert!(group.contains(&format!("Section /o \"{}\" SecLang_{}\n  !insertmacro AylisSetup \"{}\"", nsis_text(language.name()), language.folder(), language.folder())), "{}", language.name());
        }
        assert!(!group.contains("\"Rust\""), "o Rust não é opção");
        let targets = &text[text.find("SectionGroup /e \"Plataformas de destino\"").unwrap()..];
        assert!(targets.contains("!insertmacro AylisSetup \"rust --target android\"") && targets.contains("!insertmacro AylisSetup \"rust --target web\""));
        // A ordem: copiar, as origens, o Rust, as linguagens, as plataformas, o que falhou.
        let order = ["SecIde", "Section \"-Origens\"", "Section \"-Rust\"", "\"Outras linguagens\"", "\"Plataformas de destino\"", "Section \"-O que não se configurou\""];
        let at: Vec<usize> = order.iter().map(|o| text.find(o).unwrap_or_else(|| panic!("falta {o}"))).collect();
        assert!(at.windows(2).all(|w| w[0] < w[1]), "{at:?}");
        // A linha de comando marca pelos nomes.
        assert!(text.contains("\"/LINGUAGENS=\"") && text.contains("\"/PLATAFORMAS=\"") && text.contains("\",python,\"") && text.contains("\",android,\""));
        assert!(!text.contains("${GetOptions} $R0 \"/ESPELHO=\""), "o GetOptions cortaria a URL");
    }

    /// O `setup` roda direto o aylis-tool instalado (sem `cmd` nem shell), a pasta `pacotes`
    /// ao lado do instalador vale pela variável, e a falha não desfaz a IDE.
    #[test]
    fn the_script_sets_up_without_a_shell() {
        let text = generated_script();
        let exec: Vec<&str> = text.lines().filter(|l| l.contains("nsExec::")).collect();
        assert!(exec.iter().any(|l| l.contains("'\"$INSTDIR\\aylis-tool.exe\" setup ${ARGS} --utf16-output'")), "{exec:#?}");
        assert!(exec.iter().any(|l| l.contains("packages sources --add-mirror \"$Mirror\"")));
        assert!(text.contains("SetEnvironmentVariable(t \"AYLIS_PACKAGE_DIR\", t \"$EXEDIR\\pacotes\")"));
        assert!(!text.to_lowercase().contains("cmd /c") && !text.to_lowercase().contains("cmd.exe") && !text.contains("powershell"));
        let failed = &text[text.find("Section \"-O que não se configurou\"").unwrap()..];
        let failed = &failed[..failed.find("SectionEnd").unwrap()];
        assert!(failed.contains("Configurar linguagens") && failed.contains("/SD IDOK") && failed.contains("SetErrorLevel 3") && !failed.contains("Abort"));
    }

    /// A cópia e a remoção não escrevem uma linha por arquivo na lista de detalhes (era o que
    /// deixava a desinstalação lenta).
    #[test]
    fn the_script_does_not_list_every_file() {
        let text = generated_script();
        let uninstall = &text[text.find("Section \"Uninstall\"").unwrap()..];
        let quiet = uninstall.find("SetDetailsPrint textonly").expect("a remoção sem a lista");
        assert!(quiet < uninstall.find("RMDir /r \"$INSTDIR\\aylis\"").unwrap() && quiet < uninstall.find("$PROFILE\\.aylis\\target").unwrap());
        let install = &text[text.find("SecIde").unwrap()..];
        assert!(install.find("SetDetailsPrint textonly").unwrap() < install.find("RMDir /r").unwrap());
    }

    /// O desinstalador leva os pacotes e o cache do Aylis, quieto como o resto, e a pasta
    /// `.aylis` só se ficar vazia (as Opções ficam).
    #[test]
    fn the_uninstaller_removes_the_packages_and_the_cache() {
        let text = generated_script();
        let uninstall = &text[text.find("Section \"Uninstall\"").unwrap()..];
        let quiet = uninstall.find("SetDetailsPrint textonly").unwrap();
        let tools = uninstall.find("RMDir /r \"$PROFILE\\.aylis\\tools\"").expect("os pacotes");
        let cache = uninstall.find("RMDir /r \"$PROFILE\\.aylis\\downloads\"").expect("o cache");
        let base = uninstall.find("RMDir \"$PROFILE\\.aylis\"").expect("a pasta, se vazia");
        assert!(quiet < tools && tools < base && cache < base);
        assert!(!uninstall.contains("RMDir /r \"$PROFILE\\.aylis\"\n"), "nunca a pasta inteira");
        assert!(!uninstall.lines().any(|l| l.contains("tools.json") && !l.trim_start().starts_with(';')), "as Opções ficam");
    }

    #[test]
    fn nsis_texts_are_escaped() {
        assert_eq!(nsis_text("a \"b\" $c\nd"), "a $\\\"b$\\\" $$c d");
    }
}
