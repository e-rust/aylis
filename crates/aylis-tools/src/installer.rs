//! O instalador da IDE para Windows (`aylis-tool installer-ide`): compila em release, gera
//! o script do NSIS e chama o `makensis`.
//!
//! O que vai ser instalado é escolhido nos componentes do instalador:
//!
//! ```text
//! IDE (obrigatório)          aylis-ide, aylis-host, aylis-tool + aylis\resources (fontes, ícones, Lottie)
//! Projetos em Rust           aylis\Cargo.toml, Cargo.lock, crates, examples (menos IBGE e ERP), third_party (os projetos compilam contra eles)
//! Clientes das linguagens    aylis\clients (TypeScript, C#, Java, Go, C++, Free Pascal, Python)
//! Documentação               aylis\docs, schema, README.md
//! Atalho na área de trabalho
//! ```
//!
//! Instala por usuário (`%LOCALAPPDATA%\Programs\aylis`, sem pedir administrador); as
//! compilações dos projetos ficam fora da instalação (`crate::shared_target_dir`). A IDE
//! acha a pasta `aylis\` sozinha, ao lado do executável (`aylis_project::codegen::aylis_root`).

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::env::exe;

/// Os executáveis (o pacote do cargo e o nome do binário).
const BINARIES: &[(&str, &str)] = &[("aylis-ide", "aylis-ide"), ("aylis-host", "aylis-host"), ("aylis-tools", "aylis-tool"), ("aylis-migrate", "aylis-migrate")];

/// Os exemplos que ficam fora do instalador (grandes e só de demonstração: o IBGE com os
/// dados e as bandeiras, o ERP com mil forms). Continuam no repositório.
const EXAMPLES_LEFT_OUT: &[&str] = &["IBGE", "ERP"];

/// O `Cargo.toml` do workspace sem os membros de [`EXAMPLES_LEFT_OUT`].
fn without_examples(manifest: &str) -> String {
    let mut text = manifest.to_string();
    for name in EXAMPLES_LEFT_OUT {
        text = text.replace(&format!("\"examples/{name}/rust\", "), "").replace(&format!(", \"examples/{name}/rust\""), "");
    }
    text
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
    Ok(setup)
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
    // O `third_party` também: o cargo exige as dependências por caminho mesmo de outro alvo
    // (a cópia do `accesskit_android`), senão nenhum projeto em Rust compila.
    // Os exemplos, um por um, sem os de [`EXAMPLES_LEFT_OUT`]; o `Cargo.toml` é a cópia sem
    // eles entre os membros (senão o cargo recusa o workspace instalado).
    let examples: String = std::fs::read_dir(root.join("examples"))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| !EXAMPLES_LEFT_OUT.iter().any(|x| e.file_name() == *x))
        .map(|e| if e.path().is_dir() { format!("  File /r {skip}\"{}\"\n", p(&e.path())) } else { format!("  File \"{}\"\n", p(&e.path())) })
        .collect();
    let rust = format!(
        "  File \"{}\"\n{}{}  SetOutPath \"$INSTDIR\\aylis\\examples\"\n{examples}  SetOutPath \"$INSTDIR\\aylis\"\n{}",
        p(manifest),
        file("Cargo.lock"),
        tree("crates"),
        tree("third_party")
    );
    let docs = format!("{}{}{}", tree("docs"), tree("schema"), file("README.md"));
    let file_icons: String = crate::icons::FILE_TYPES.iter().map(|(ext, _, _)| format!("  File \"{}\"\n", p(&icons.join(format!("{ext}.ico"))))).collect();
    TEMPLATE
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
        .replace("@RUST@", &rust)
        .replace("@CLIENTS@", &tree("clients"))
        .replace("@DOCS@", &docs)
}

/// O script do NSIS (Modern UI, em português); os `@...@` são trocados em `script`.
const TEMPLATE: &str = r#"Unicode true
!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "WinMessages.nsh"
!include "StrFunc.nsh"
; As funções de texto usadas (o StrFunc pede a declaração antes).
${StrStr}
${UnStrRep}

!define UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\aylis-ide"

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
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "PortugueseBR"

Section "IDE" SecIde
  SectionIn RO
  ; Uma versão anterior sai inteira (sem sobrar arquivos que a nova não tem).
  RMDir /r "$INSTDIR\aylis"
  SetOutPath "$INSTDIR"
@BINS@@README@
  SetOutPath "$INSTDIR\aylis"
@RESOURCES@
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

Section "Projetos em Rust" SecRust
  SetOutPath "$INSTDIR\aylis"
@RUST@SectionEnd

Section "Clientes das outras linguagens" SecClients
  SetOutPath "$INSTDIR\aylis"
@CLIENTS@SectionEnd

Section "Documentação" SecDocs
  SetOutPath "$INSTDIR\aylis"
@DOCS@SectionEnd

Section /o "Atalho na área de trabalho" SecDesktop
  CreateShortcut "$DESKTOP\Aylis IDE.lnk" "$INSTDIR\aylis-ide.exe"
SectionEnd

; O tamanho em "Aplicativos instalados" (depois de tudo copiado).
Section "-Tamanho"
  ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
  WriteRegDWORD HKCU "${UNINST_KEY}" "EstimatedSize" $0
SectionEnd

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecIde} "A IDE, o aylis-host (projetos em Python e nas outras linguagens), o aylis-tool e as fontes e ícones."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecRust} "O código do aylis: os projetos em Rust compilam contra ele. Precisa do Rust (rustup) instalado."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecClients} "Os clientes do protocolo em TypeScript, C#, Java, Go, C++ e Free Pascal."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDocs} "A documentação do Aylis e os esquemas JSON dos forms."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDesktop} "Um atalho da IDE na área de trabalho."
!insertmacro MUI_FUNCTION_DESCRIPTION_END

Section "Uninstall"
  Delete "$DESKTOP\Aylis IDE.lnk"
  RMDir /r "$SMPROGRAMS\Aylis"
  RMDir /r "$INSTDIR\aylis"
  RMDir /r "$INSTDIR\icons"
@UNBINS@  Delete "$INSTDIR\LEIA-ME.txt"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\desinstalar.exe"
  RMDir "$INSTDIR"
  ; As compilações dos projetos (cache; refeitas sob demanda).
  RMDir /r "$PROFILE\.aylis\target"
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
  aylis-tool      Executar/Publicar na linha de comando (aylis-tool check | run | publish)
  aylis\\          fontes e ícones; com \"Projetos em Rust\", o código do aylis (os projetos compilam contra ele)

Para projetos em Rust é preciso o Rust instalado (rustup); a IDE confere o ambiente antes
de executar. As compilações ficam em %USERPROFILE%\\.aylis\\target (outra pasta nas Opções da IDE),
compartilhadas entre os projetos; o que nenhum projeto usa mais sai sozinho (ou com
aylis-tool clean).
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_installed_workspace_leaves_the_big_examples_out() {
        let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.toml")).unwrap();
        assert!(manifest.contains("examples/IBGE/rust"), "o repositório tem o IBGE");
        let installed = without_examples(&manifest);
        assert!(!installed.contains("examples/IBGE/rust"));
        assert!(installed.contains("examples/Calculadora/rust"));
        assert!(!installed.contains(", ,") && !installed.contains("[, "), "a lista continua válida");
    }
}
