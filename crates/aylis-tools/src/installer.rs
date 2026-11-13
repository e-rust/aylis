//! O instalador da IDE para Windows (`aylis-tool installer-ide`): compila em release, gera
//! o script do NSIS e chama o `makensis`.
//!
//! O que vai ser instalado é escolhido nos componentes do instalador:
//!
//! ```text
//! IDE (obrigatório)          aylis-ide, aylis-host, aylis-tool + aylis\resources (fontes, ícones, Lottie)
//! Projetos em Rust           aylis\Cargo.toml, Cargo.lock, crates, examples (menos IBGE e ERP, e sem o generated/), third_party (os projetos compilam contra eles)
//! Clientes das linguagens    aylis\clients (TypeScript, C#, Java, Go, C++, Free Pascal, Python)
//! Documentação               aylis\docs, schema, README.md
//! Atalho na área de trabalho
//! ```
//!
//! Instala por usuário (`%LOCALAPPDATA%\Programs\aylis`, sem pedir administrador); as
//! compilações dos projetos ficam fora da instalação (`crate::shared_target_dir`). A IDE
//! acha a pasta `aylis\` sozinha, ao lado do executável (`aylis_project::codegen::aylis_root`).
//!
//! ## Os pacotes de ferramentas (`docs/pacotes.md`)
//!
//! Depois dos componentes vem uma segunda página com os pacotes do manifesto embutido
//! ([`crate::toolpacks::Manifest`]) para Windows x86_64, um por caixa, com o título e o
//! tamanho; a lista é gerada do manifesto a cada `installer-ide`. Depois dela, se algum foi
//! marcado, a página do espelho (o endereço HTTPS de um `pacotes.json`, opcional). No fim da
//! instalação, o `aylis-tool` recém-instalado grava o espelho nas Opções
//! (`packages sources --add-mirror`) e instala cada pacote marcado
//! (`packages install <nome> --from "<pasta do setup>\pacotes"`), com a saída nos detalhes da
//! instalação (`nsExec`, sem `cmd`). O pacote que falhar (sem origem, hash que não confere) é
//! dito no fim, com o caminho para instalar depois (Ferramentas → Pacotes), e a IDE fica.
//!
//! Nenhum pacote vem marcado, com uma exceção: o `rust`, quando a pasta `pacotes` ao lado do
//! instalador tem os arquivos dele, "Projetos em Rust" está marcado e a máquina não tem o
//! rustup (`%USERPROFILE%\.cargo\bin\rustc.exe`). É o caso do kit offline: quem o recebe
//! quer compilar sem internet. Fora dele, marcar um pacote sem origem só daria erro no fim.
//!
//! Em modo silencioso (ou para já abrir marcado):
//!
//! ```text
//! aylis-ide-<versão>-setup.exe /S /PACOTES=rust,jdk /ESPELHO=https://exemplo/pacotes.json /D=C:\pasta
//! ```
//!
//! `/PACOTES=` aceita os nomes do manifesto separados por vírgula (sem espaços) ou `all`; um
//! nome que não existe é ignorado. O código de saída é 3 quando a IDE foi instalada e algum
//! pacote não. O desinstalador remove também os pacotes (`%USERPROFILE%\.aylis\tools`), o
//! cache deles (`downloads`) e o `environment.json`; as Opções (`tools.json`) ficam.
//!
//! ## O kit offline
//!
//! `aylis-tool installer-ide --with-packages node,codelldb` (ou `all`, ou sem nomes: todos)
//! gera o setup e, ao lado dele, a pasta `pacotes` (o `packages pack` para Windows x86_64,
//! com as dependências). A pasta e o setup juntos (no Google Drive, numa pasta de rede, num
//! pendrive) instalam tudo sem internet.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::env::exe;
use crate::toolpacks::{Manifest, Package};

/// A plataforma do instalador (os pacotes da página e do kit).
const PLATFORM: &str = "windows-x86_64";

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

/// Os pacotes do manifesto que existem para o instalador (Windows x86_64), na ordem dele.
fn installer_packages(manifest: &Manifest) -> Vec<Package> {
    manifest.names().iter().filter_map(|n| manifest.package_for(n, PLATFORM)).filter(Package::available).collect()
}

/// Um texto entre aspas duplas no script do NSIS (`$` e `"` escapados, numa linha só).
fn nsis_text(s: &str) -> String {
    s.replace('$', "$$").replace('"', "$\\\"").replace(['\r', '\n'], " ")
}

/// As partes do script com os pacotes: as seções, as descrições, mostrar e esconder (as duas
/// páginas de componentes), os marcados ao abrir e "algum marcado?".
struct PackageScript {
    sections: String,
    descriptions: String,
    show: String,
    hide: String,
    from_parameter: String,
    default: String,
    any: String,
}

fn package_script(manifest: &Manifest) -> PackageScript {
    let mut s = PackageScript { sections: String::new(), descriptions: String::new(), show: String::new(), hide: String::new(), from_parameter: String::new(), default: String::new(), any: String::new() };
    for (i, p) in installer_packages(manifest).iter().enumerate() {
        let id = format!("SecPkg{i}");
        let label = nsis_text(&format!("{} ({} MB)", p.title, p.size().div_ceil(1 << 20)));
        // O nome vai para a linha de comando: só os do manifesto (já conferidos por ele).
        assert!(crate::toolpacks::plain_name(&p.name), "nome de pacote inválido: {}", p.name);
        s.sections.push_str(&format!(
            "Section /o \"{label}\" {id}\n  AddSize {}\n  !insertmacro AylisPackage \"{}\" \"{}\"\nSectionEnd\n\n",
            p.size().div_ceil(1024),
            p.name,
            nsis_text(&p.title)
        ));
        let needs = if p.depends.is_empty() { String::new() } else { format!(" Instala junto: {}.", p.depends.join(", ")) };
        s.descriptions.push_str(&format!("  !insertmacro MUI_DESCRIPTION_TEXT ${{{id}}} \"{}{}\"\n", nsis_text(&p.description), nsis_text(&needs)));
        s.show.push_str(&format!("  SectionSetText ${{{id}}} \"{label}\"\n"));
        s.hide.push_str(&format!("  SectionSetText ${{{id}}} \"\"\n"));
        s.from_parameter.push_str(&format!(
            "  ${{StrStr}} $R2 \"$R1\" \",{name},\"\n  StrCmp $R2 \"\" 0 aylis_pkg_on_{i}\n  ${{StrStr}} $R2 \"$R1\" \",all,\"\n  StrCmp $R2 \"\" aylis_pkg_next_{i}\n  aylis_pkg_on_{i}:\n  !insertmacro SelectSection ${{{id}}}\n  aylis_pkg_next_{i}:\n",
            name = p.name
        ));
        s.any.push_str(&format!("  SectionGetFlags ${{{id}}} $R8\n  IntOp $R8 $R8 & ${{SF_SELECTED}}\n  IntOp $R9 $R9 | $R8\n"));
        // O Rust já marcado no kit offline (veja o módulo).
        if p.name == "rust" {
            let files: String = p.files.iter().map(|f| format!("  IfFileExists \"$EXEDIR\\pacotes\\{}\" 0 aylis_packages_done\n", f.name)).collect();
            s.default = format!(
                "  SectionGetFlags ${{SecRust}} $R2\n  IntOp $R2 $R2 & ${{SF_SELECTED}}\n  IntCmp $R2 0 aylis_packages_done\n  IfFileExists \"$PROFILE\\.cargo\\bin\\rustc.exe\" aylis_packages_done\n{files}  !insertmacro SelectSection ${{{id}}}\n"
            );
        }
    }
    s
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
        // Sem o `generated/`: o Aylis o refaz sozinho ao gerar, compilar ou executar (a metade
        // dos arquivos dos exemplos, que pesavam na cópia e na remoção).
        .map(|e| if e.path().is_dir() { format!("  File /r {skip}/x generated \"{}\"\n", p(&e.path())) } else { format!("  File \"{}\"\n", p(&e.path())) })
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
    let packages = package_script(&Manifest::embedded());
    TEMPLATE
        .replace("@PACKAGE_SECTIONS@", &packages.sections)
        .replace("@PACKAGE_DESCRIPTIONS@", &packages.descriptions)
        .replace("@PACKAGE_SHOW@", &packages.show)
        .replace("@PACKAGE_HIDE@", &packages.hide)
        .replace("@PACKAGE_PARAMETER@", &packages.from_parameter)
        .replace("@PACKAGE_DEFAULT@", &packages.default)
        .replace("@PACKAGE_ANY@", &packages.any)
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
!include "Sections.nsh"
!include "nsDialogs.nsh"
; As funções de texto usadas (o StrFunc pede a declaração antes).
${StrStr}
${UnStrRep}

!define UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\aylis-ide"

; Os nomes das seções da primeira página (a segunda, a dos pacotes, as esconde e volta).
!define TEXT_IDE "IDE"
!define TEXT_RUST "Projetos em Rust"
!define TEXT_CLIENTS "Clientes das outras linguagens"
!define TEXT_DOCS "Documentação"
!define TEXT_DESKTOP "Atalho na área de trabalho"

; O espelho dos pacotes (a página ou /ESPELHO=), os pacotes que falharam e o campo da página.
Var Mirror
Var PackagesFailed
Var MirrorField

; Um pacote de ferramentas: o aylis-tool recém-instalado o instala (e as dependências) da
; pasta `pacotes` ao lado do setup, das origens das Opções e do espelho gravado. A saída vai
; para os detalhes (em UTF-16: o nsExec lê UTF-8 como ANSI). Falhar não desfaz a IDE.
!macro AylisPackage NAME TITLE
  SetDetailsPrint both
  DetailPrint "Instalando o pacote ${TITLE}…"
  nsExec::ExecToLog '"$INSTDIR\aylis-tool.exe" packages install ${NAME} --from "$EXEDIR\pacotes" --utf16-output'
  Pop $R0
  StrCmp $R0 "0" +2
  StrCpy $PackagesFailed "$PackagesFailed, ${TITLE} (${NAME})"
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
!define MUI_PAGE_CUSTOMFUNCTION_PRE AylisComponentsPre
!insertmacro MUI_PAGE_COMPONENTS
; A segunda página de componentes: só os pacotes de ferramentas (como no Delphi).
!define MUI_PAGE_HEADER_TEXT "Pacotes de ferramentas"
!define MUI_PAGE_HEADER_SUBTEXT "Marque as ferramentas que os seus projetos vão usar."
!define MUI_COMPONENTSPAGE_TEXT_TOP "Os marcados se instalam no fim, da pasta pacotes ao lado do instalador ou do espelho da página seguinte. O resto se instala depois, na IDE (Ferramentas → Pacotes). O que a máquina já tem não precisa de pacote."
!define MUI_COMPONENTSPAGE_TEXT_COMPLIST "Pacotes:"
!define MUI_PAGE_CUSTOMFUNCTION_PRE AylisPackagesPre
!insertmacro MUI_PAGE_COMPONENTS
Page custom AylisMirrorPage AylisMirrorLeave
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "PortugueseBR"

Section "${TEXT_IDE}" SecIde
  SectionIn RO
  ; Os milhares de arquivos sem a linha de cada um na lista de detalhes: escrever na lista é o
  ; que mais custa (medido em 12/11: 20 mil arquivos apagados em 28,3 s com a lista e 3,1 s
  ; sem ela). A barra mostra só a etapa.
  SetDetailsPrint textonly
  DetailPrint "Removendo a versão anterior…"
  ; Uma versão anterior sai inteira (sem sobrar arquivos que a nova não tem).
  RMDir /r "$INSTDIR\aylis"
  DetailPrint "Copiando a IDE…"
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

Section "${TEXT_RUST}" SecRust
  DetailPrint "Copiando o código do Aylis e os exemplos…"
  SetOutPath "$INSTDIR\aylis"
@RUST@SectionEnd

Section "${TEXT_CLIENTS}" SecClients
  DetailPrint "Copiando os clientes das outras linguagens…"
  SetOutPath "$INSTDIR\aylis"
@CLIENTS@SectionEnd

Section "${TEXT_DOCS}" SecDocs
  DetailPrint "Copiando a documentação…"
  SetOutPath "$INSTDIR\aylis"
@DOCS@SectionEnd

Section /o "${TEXT_DESKTOP}" SecDesktop
  CreateShortcut "$DESKTOP\Aylis IDE.lnk" "$INSTDIR\aylis-ide.exe"
SectionEnd

; O tamanho em "Aplicativos instalados" (depois de tudo copiado).
Section "-Tamanho"
  ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
  WriteRegDWORD HKCU "${UNINST_KEY}" "EstimatedSize" $0
SectionEnd

; O espelho informado vai para as Opções (pelo aylis-tool, como a IDE grava), antes dos
; pacotes: eles o usam para o que não estiver na pasta `pacotes`.
Section "-Espelho"
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

; Os pacotes de ferramentas (a segunda página), gerados do manifesto embutido.
@PACKAGE_SECTIONS@; O que não se instalou: a IDE fica, e o pacote se instala depois pela IDE.
Section "-Pacotes que falharam"
  StrCmp $PackagesFailed "" aylis_packages_ok
  StrCpy $R0 $PackagesFailed "" 2
  SetDetailsPrint both
  DetailPrint "Não se instalaram: $R0. Instale-os depois na IDE, em Ferramentas → Pacotes."
  MessageBox MB_ICONEXCLAMATION|MB_OK "Não deu para instalar estes pacotes: $R0.$\r$\n$\r$\nA IDE foi instalada. O motivo está nos detalhes da instalação (o arquivo não está na pasta pacotes ao lado do instalador nem no espelho, ou não confere com o SHA-256 do Aylis).$\r$\n$\r$\nInstale-os depois na IDE, em Ferramentas → Pacotes (ou com aylis-tool packages install)." /SD IDOK
  SetErrorLevel 3
  aylis_packages_ok:
SectionEnd

; A primeira página de componentes: a IDE e o resto, sem os pacotes.
Function AylisComponentsPre
  SectionSetText ${SecIde} "${TEXT_IDE}"
  SectionSetText ${SecRust} "${TEXT_RUST}"
  SectionSetText ${SecClients} "${TEXT_CLIENTS}"
  SectionSetText ${SecDocs} "${TEXT_DOCS}"
  SectionSetText ${SecDesktop} "${TEXT_DESKTOP}"
@PACKAGE_HIDE@FunctionEnd

; A segunda: só os pacotes (uma seção sem texto não aparece na lista).
Function AylisPackagesPre
  SectionSetText ${SecIde} ""
  SectionSetText ${SecRust} ""
  SectionSetText ${SecClients} ""
  SectionSetText ${SecDocs} ""
  SectionSetText ${SecDesktop} ""
@PACKAGE_SHOW@FunctionEnd

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

; A página do espelho: só se algum pacote foi marcado.
Function AylisMirrorPage
  StrCpy $R9 0
@PACKAGE_ANY@  StrCmp $R9 0 0 +2
  Abort
  !insertmacro MUI_HEADER_TEXT "Espelho dos pacotes" "De onde vêm os pacotes que não estiverem ao lado do instalador (opcional)."
  nsDialogs::Create 1018
  Pop $R0
  IfFileExists "$EXEDIR\pacotes\*.*" 0 aylis_no_folder
  StrCpy $R1 "A pasta de pacotes ao lado do instalador foi encontrada ($EXEDIR\pacotes): os pacotes marcados vêm dela, e o espelho fica para o que faltar."
  Goto aylis_folder_done
  aylis_no_folder:
  StrCpy $R1 "Não há a pasta pacotes ao lado do instalador ($EXEDIR\pacotes): sem um espelho, os pacotes marcados não terão de onde vir (dá para instalá-los depois, na IDE)."
  aylis_folder_done:
  ${NSD_CreateLabel} 0 0 100% 30u "$R1"
  Pop $R0
  ${NSD_CreateLabel} 0 36u 100% 20u "Espelho dos pacotes: o link da pasta compartilhada do Google Drive ou o endereço HTTPS de um pacotes.json (de quem distribui o Aylis). Fica gravado nas Opções da IDE."
  Pop $R0
  ${NSD_CreateText} 0 60u 100% 12u "$Mirror"
  Pop $MirrorField
  ${NSD_CreateLabel} 0 80u 100% 30u "Os arquivos são conferidos com o SHA-256 que o Aylis traz, venham de onde vierem."
  Pop $R0
  nsDialogs::Show
FunctionEnd

Function AylisMirrorLeave
  ${NSD_GetText} $MirrorField $Mirror
  Call AylisCheckMirror
  StrCmp $R0 "" aylis_leave_ok
  MessageBox MB_ICONEXCLAMATION|MB_OK "Espelho dos pacotes: $R0."
  Abort
  aylis_leave_ok:
FunctionEnd

; /PACOTES=rust,jdk (ou all) e /ESPELHO=https://... na linha de comando (o modo silencioso,
; /S, usa só eles). Sem /PACOTES=, nada marcado, menos o Rust do kit offline (veja o módulo).
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
  ClearErrors
  ${GetOptions} $R0 "/PACOTES=" $R1
  IfErrors aylis_packages_default
  StrCpy $R1 ",$R1,"
@PACKAGE_PARAMETER@  Goto aylis_packages_done
  aylis_packages_default:
@PACKAGE_DEFAULT@  aylis_packages_done:
FunctionEnd

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecIde} "A IDE, o aylis-host (projetos em Python e nas outras linguagens), o aylis-tool e as fontes e ícones."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecRust} "O código do aylis: os projetos em Rust compilam contra ele. Precisa do Rust (rustup) instalado."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecClients} "Os clientes do protocolo em TypeScript, C#, Java, Go, C++ e Free Pascal."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDocs} "A documentação do Aylis e os esquemas JSON dos forms."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDesktop} "Um atalho da IDE na área de trabalho."
@PACKAGE_DESCRIPTIONS@!insertmacro MUI_FUNCTION_DESCRIPTION_END

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
  aylis-tool      Executar/Publicar na linha de comando (aylis-tool check | run | publish)
  aylis\\          fontes e ícones; com \"Projetos em Rust\", o código do aylis (os projetos compilam contra ele)

Para projetos em Rust é preciso o Rust instalado (rustup); a IDE confere o ambiente antes
de executar. As compilações ficam em %USERPROFILE%\\.aylis\\target (outra pasta nas Opções da IDE),
compartilhadas entre os projetos; o que nenhum projeto usa mais sai sozinho (ou com
aylis-tool clean).

Pacotes de ferramentas
  O instalador mostra, depois dos componentes, os pacotes (o Rust, o JDK, o SDK e o NDK do
  Android, o Python, o Node, o .NET, o Go, os depuradores...). Os marcados se instalam no fim,
  em %USERPROFILE%\\.aylis\\tools, da pasta \"pacotes\" ao lado do instalador ou do espelho
  informado (o endereço HTTPS de um pacotes.json; fica gravado nas Opções da IDE). Cada
  arquivo é conferido com o SHA-256 que o Aylis traz. O que falhar se instala depois, na IDE
  (Ferramentas -> Pacotes) ou com: aylis-tool packages install <nome>

  Sem janelas (modo silencioso):
    aylis-ide-<versão>-setup.exe /S /PACOTES=rust,jdk /ESPELHO=https://.../pacotes.json /D=C:\\pasta
  /PACOTES= recebe os nomes separados por vírgula, sem espaços (aylis-tool packages list
  mostra os nomes), ou all. Sai com o código 3 se a IDE foi instalada e algum pacote não.

  O kit offline (para distribuir tudo junto, numa pasta de rede ou no Google Drive):
    aylis-tool installer-ide --with-packages rust,jdk      (ou all)
  grava o setup e, ao lado dele, a pasta \"pacotes\" com os arquivos e as dependências. Copie
  os dois juntos; quem roda o setup instala sem internet.

  O desinstalador remove também os pacotes do Aylis e o cache deles
  (%USERPROFILE%\\.aylis\\tools e downloads); o que a máquina já tinha não é tocado.
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

    /// Os exemplos vão sem o `generated/` (o Aylis refaz), e a cópia e a remoção não escrevem
    /// uma linha por arquivo na lista de detalhes (era o que deixava a desinstalação lenta).
    #[test]
    fn the_script_leaves_generated_out_and_does_not_list_every_file() {
        let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let x = Path::new("x");
        let text = script(root, x, x, x, x, x, "0.0.0");
        let examples: Vec<&str> = text.lines().filter(|l| l.contains("File /r") && l.contains("\\examples\\")).collect();
        assert!(examples.iter().any(|l| l.contains("Calculadora")), "{text}");
        assert!(examples.iter().all(|l| l.contains("/x generated")), "{examples:#?}");
        assert!(!text.lines().any(|l| l.contains("\\crates\"") && l.contains("/x generated")), "só nos exemplos");
        let uninstall = &text[text.find("Section \"Uninstall\"").unwrap()..];
        let quiet = uninstall.find("SetDetailsPrint textonly").expect("a remoção sem a lista");
        assert!(quiet < uninstall.find("RMDir /r \"$INSTDIR\\aylis\"").unwrap() && quiet < uninstall.find("$PROFILE\\.aylis\\target").unwrap());
        let install = &text[text.find("Section \"${TEXT_IDE}\"").unwrap()..];
        assert!(install.find("SetDetailsPrint textonly").unwrap() < install.find("RMDir /r").unwrap());
    }

    fn generated_script() -> String {
        let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let x = Path::new("x");
        script(root, x, x, x, x, x, "0.0.0")
    }

    /// A página dos pacotes sai do manifesto embutido: uma seção desmarcada por pacote do
    /// Windows, com o título e o tamanho, e nada sobra dos marcadores do modelo.
    #[test]
    fn the_script_has_a_page_with_the_manifest_packages() {
        let text = generated_script();
        assert!(!text.contains("@PACKAGE"), "marcador sem trocar");
        let packages = installer_packages(&Manifest::embedded());
        assert!(packages.iter().any(|p| p.name == "rust") && packages.iter().any(|p| p.name == "jdk"), "{packages:#?}");
        for (i, p) in packages.iter().enumerate() {
            let header = format!("Section /o \"{} ({} MB)\" SecPkg{i}", nsis_text(&p.title), p.size().div_ceil(1 << 20));
            assert!(text.contains(&header), "falta {header}");
            assert!(text.contains(&format!("!insertmacro AylisPackage \"{}\"", p.name)));
            assert!(text.contains(&format!("\",{},\"", p.name)), "/PACOTES= reconhece {}", p.name);
        }
        // Duas páginas de componentes (a segunda só com os pacotes) e a do espelho.
        assert_eq!(text.matches("!insertmacro MUI_PAGE_COMPONENTS").count(), 2);
        assert!(text.contains("MUI_PAGE_CUSTOMFUNCTION_PRE AylisPackagesPre") && text.contains("Page custom AylisMirrorPage AylisMirrorLeave"));
        assert!(text.contains("\"/PACOTES=\"") && text.contains("\"/ESPELHO=\""));
        // O GetOptions cortaria a URL no primeiro "/" (sobrava "https:").
        assert!(!text.contains("${GetOptions} $R0 \"/ESPELHO=\""));
        // Os pacotes vêm depois de tudo copiado e do espelho gravado.
        let size = text.find("Section \"-Tamanho\"").unwrap();
        let mirror = text.find("Section \"-Espelho\"").unwrap();
        let first = text.find("SecPkg0\n").unwrap();
        let failed = text.find("Section \"-Pacotes que falharam\"").unwrap();
        assert!(size < mirror && mirror < first && first < failed);
        // Só o Rust pode vir marcado (o kit offline), conferindo cada arquivo dele.
        let rust = Manifest::embedded().package_for("rust", PLATFORM).unwrap();
        for f in &rust.files {
            assert!(text.contains(&format!("IfFileExists \"$EXEDIR\\pacotes\\{}\"", f.name)));
        }
    }

    /// O `packages install` roda direto o aylis-tool instalado (sem `cmd` nem shell), com a
    /// pasta `pacotes` ao lado do setup, e a falha não desfaz a IDE.
    #[test]
    fn the_script_installs_the_packages_without_a_shell() {
        let text = generated_script();
        let exec: Vec<&str> = text.lines().filter(|l| l.contains("nsExec::")).collect();
        assert!(exec.iter().any(|l| l.contains("'\"$INSTDIR\\aylis-tool.exe\" packages install ${NAME} --from \"$EXEDIR\\pacotes\" --utf16-output'")), "{exec:#?}");
        assert!(exec.iter().any(|l| l.contains("packages sources --add-mirror \"$Mirror\"")));
        assert!(!text.to_lowercase().contains("cmd /c") && !text.to_lowercase().contains("cmd.exe") && !text.contains("powershell"));
        let failed = &text[text.find("Section \"-Pacotes que falharam\"").unwrap()..];
        assert!(failed.contains("Ferramentas → Pacotes") && failed.contains("/SD IDOK") && !failed[..failed.find("SectionEnd").unwrap()].contains("Abort"));
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
