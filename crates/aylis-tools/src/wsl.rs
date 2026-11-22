//! Linux a partir do Windows, pelo WSL: compilar e executar dentro de uma distribuição (a
//! janela aparece pelo WSLg) e publicar o executável e um pacote `.deb`.
//!
//! O projeto fica onde está, no Windows. Dentro do Linux a IDE monta uma pasta-espelho em
//! `~/.aylis-wsl/<pacote>`: o `Cargo.toml` com os caminhos do Windows (`C:/...`) trocados
//! pelos do Linux (`/mnt/c/...`) e o resto como links para a pasta do projeto. O que o cargo
//! compila fica em `~/.aylis-wsl/target`, separado do `target` do Windows.

use std::path::Path;
use std::process::Command;

use aylis_project::Project;

use crate::process::{Control, Event, LineKind, Sink, run_lines};

/// As bibliotecas do Linux que o aylis usa para compilar (janela, teclado, fontes, OpenGL,
/// áudio).
pub const PACKAGES: [&str; 16] = [
    "build-essential",
    "pkg-config",
    "curl",
    "libx11-dev",
    "libxcursor-dev",
    "libxrandr-dev",
    "libxi-dev",
    "libxkbcommon-dev",
    "libxkbcommon-x11-dev",
    "libwayland-dev",
    "libfontconfig1-dev",
    "libfreetype-dev",
    "libgl1-mesa-dev",
    "libegl1-mesa-dev",
    // Áudio (o aylis toca sons e o inspetor do app vem com a mídia).
    "libasound2-dev",
    // Controles de jogo (o `Gamepad`, pelo gilrs).
    "libudev-dev",
];

/// Saída de um `wsl.exe` (que fala UTF-16 nas listas).
fn decode(bytes: &[u8]) -> String {
    if bytes.len() >= 2 && bytes[1] == 0 {
        let units: Vec<u16> = bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        String::from_utf16_lossy(&units)
    } else {
        String::from_utf8_lossy(bytes).into_owned()
    }
}

fn wsl() -> Command {
    #[cfg_attr(not(windows), allow(unused_mut))]
    let mut command = Command::new("wsl.exe");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    command
}

/// O prazo das perguntas ao WSL (a primeira acorda a máquina virtual, uns 4 s): um WSL travado
/// não prende a verificação de ambiente.
const ASK_LIMIT: std::time::Duration = std::time::Duration::from_secs(45);

fn ask(command: &mut Command) -> std::io::Result<std::process::Output> {
    crate::env::output_within(command, ASK_LIMIT)
}

/// O WSL está instalado (o `wsl.exe` responde).
pub fn available() -> bool {
    cfg!(windows) && ask(wsl().arg("--status")).is_ok_and(|o| o.status.success())
}

/// As distribuições instaladas, sem as internas do Docker.
pub fn distros() -> Vec<String> {
    let Ok(out) = ask(wsl().args(["-l", "-q"])) else { return Vec::new() };
    decode(&out.stdout)
        .lines()
        .map(|l| l.trim().trim_matches('\0').to_string())
        .filter(|l| !l.is_empty() && !l.starts_with("docker-desktop"))
        .collect()
}

/// A distribuição usada: `AYLIS_WSL_DISTRO`; senão a `Ubuntu-22.04` (a glibc dela é a mínima
/// dos apps do Linux, `crate::glibc`: compilado no Ubuntu 26.04, o app não abria no Mint 22,
/// 22/11); senão a primeira.
pub fn distro() -> Option<String> {
    if let Some(d) = std::env::var("AYLIS_WSL_DISTRO").ok().filter(|d| !d.is_empty()) {
        return Some(d);
    }
    pick(&distros())
}

/// A escolha entre as instaladas: a do Ubuntu 22.04, senão a primeira.
fn pick(installed: &[String]) -> Option<String> {
    installed.iter().find(|d| d.eq_ignore_ascii_case(crate::glibc::WSL_DISTRO)).or_else(|| installed.first()).cloned()
}

/// A glibc da distribuição (pelo `ldd --version`).
pub fn glibc(distro: &str) -> Option<(u32, u32)> {
    shell_output(distro, "ldd --version").as_deref().and_then(crate::glibc::parse_ldd)
}

/// Roda `script` no bash da distribuição e devolve a saída (para conferir o ambiente).
pub fn shell_output(distro: &str, script: &str) -> Option<String> {
    let out = ask(wsl().args(["-d", distro, "--exec", "bash", "-lc", script])).ok()?;
    out.status.success().then(|| decode(&out.stdout))
}

/// `C:\Users\x` → `/mnt/c/Users/x`.
pub fn to_linux(path: &Path) -> String {
    let text = path.display().to_string().replace('\\', "/");
    let text = text.trim_start_matches("//?/").to_string();
    match text.as_bytes() {
        [d, b':', rest @ ..] if d.is_ascii_alphabetic() => format!("/mnt/{}{}", (*d as char).to_ascii_lowercase(), String::from_utf8_lossy(rest)),
        _ => text,
    }
}

/// Aspas simples do bash.
fn quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

/// O começo de todo script: o cargo do rustup, a pasta-espelho (o projeto e, dentro dela, a
/// pasta do código em Rust) e o `Cargo.toml` convertido; termina na pasta do crate. No
/// `Cargo.toml`, os caminhos do Windows (`C:/...`) viram os do Linux (`/mnt/c/...`), e os
/// relativos (o `sdk` numa pasta, os exemplos do repositório: `../../crates/aylis`) passam a
/// partir da pasta do código de verdade, não da espelho (antes não achavam o Aylis, 22/11).
fn prepare(project: &Project) -> String {
    let package = project.package_name();
    format!(
        r#"set -e
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
if ! command -v cargo >/dev/null; then echo "o Rust não está instalado no Linux do WSL (Ferramentas → Verificar ambiente instala)" >&2; exit 3; fi
P={project}
C={code}
S="$HOME/.aylis-wsl/{package}"
mkdir -p "$S/$C"
for f in "$P"/* "$P"/.aylis; do
  [ -e "$f" ] || continue
  n=$(basename "$f")
  case "$n" in "$C"|target|dist) continue;; esac
  ln -sfn "$f" "$S/$n"
done
for f in "$P/$C"/*; do
  [ -e "$f" ] || continue
  n=$(basename "$f")
  case "$n" in Cargo.toml|Cargo.lock|target) continue;; esac
  ln -sfn "$f" "$S/$C/$n"
done
sed -E 's#"([A-Za-z]):/#"/mnt/\L\1/#g' "$P/$C/Cargo.toml" > "$S/$C/Cargo.toml"
sed -E -i 's#(path *= *")([^/"][^:"]*")#\1'"$P/$C"'/\2#g' "$S/$C/Cargo.toml"
export CARGO_TARGET_DIR="$HOME/.aylis-wsl/target"
cd "$S/$C"
"#,
        project = quote(&to_linux(&project.dir())),
        code = project.language.folder(),
    )
}

/// O script do Executar: compila e roda o programa (a janela aparece pelo WSLg). Sem o canal
/// da IDE: o `build.rs` roda no Linux, longe do alvo que a IDE grava no Windows.
pub fn run_script(project: &Project) -> String {
    let package = project.bin_name();
    let mut s = prepare(project);
    s.push_str(&format!("echo '▶ Compilando {} (Linux, WSL)…'\n", project.name));
    s.push_str(&format!("cargo build --bin {package}\n"));
    s.push_str(&format!("echo '▶ Executando {} (Linux, WSL)…'\n", project.name));
    // O WSLg põe estas variáveis nas sessões normais; num `wsl.exe` sem login, garante.
    s.push_str(": \"${WAYLAND_DISPLAY:=wayland-0}\" \"${DISPLAY:=:0}\" \"${XDG_RUNTIME_DIR:=/mnt/wslg/runtime-dir}\"\n");
    s.push_str("export WAYLAND_DISPLAY DISPLAY XDG_RUNTIME_DIR RUST_BACKTRACE=1\n");
    // A placa de vídeo do Windows chega ao Linux pelo `/dev/dxg`; o Mesa só a usa com o
    // driver d3d12 (sem ele, desenha pela CPU, no llvmpipe), e pelo X11: pelo Wayland do
    // WSLg o EGL com d3d12 derruba a conexão.
    s.push_str(
        "if [ -e /dev/dxg ] && [ -z \"$GALLIUM_DRIVER\" ]; then export GALLIUM_DRIVER=d3d12 LD_LIBRARY_PATH=/usr/lib/wsl/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}; unset WAYLAND_DISPLAY; fi\n",
    );
    s.push_str(&format!("cd {}\n", quote(&to_linux(&project.dir()))));
    s.push_str(&format!("exec \"$CARGO_TARGET_DIR/debug/{package}\"\n"));
    s
}

/// O começo dos scripts que usam o `aylis-tool` de Linux: compila-o (só a primeira vez demora)
/// e põe em `T`.
fn linux_tool() -> String {
    let root = quote(&to_linux(Path::new(&crate::aylis_root())));
    format!(
        "echo '▶ Preparando o aylis-tool do Linux (a primeira vez demora alguns minutos)…'\n\
         cargo build --release --manifest-path {root}/Cargo.toml -p aylis-tools --bin aylis-tool\n\
         T=\"$CARGO_TARGET_DIR/release/aylis-tool\"\n"
    )
}

/// O script da publicação: compila otimizado e copia o executável para `out` (no Windows). O
/// `.deb` sai depois, no Windows, pelo mesmo [`crate::app_linux`] do Linux (antes este script
/// montava outro, pelo `dpkg-deb`, e os dois divergiam).
pub fn publish_script(project: &Project, out: &Path, app_file: &str) -> String {
    let out_l = quote(&to_linux(out));
    let mut s = prepare(project);
    s.push_str(&format!("echo '▶ Compilando {} (Linux, otimizado)…'\n", project.name));
    let bin = project.bin_name();
    s.push_str(&format!("cargo build --release --bin {bin}\n"));
    s.push_str(&format!("mkdir -p {out_l}\ncp \"$CARGO_TARGET_DIR/release/{bin}\" {out_l}/{}\n", quote(app_file)));
    s
}

/// O script da publicação das outras linguagens: o mesmo publicar do Linux, pelo `aylis-tool`
/// de Linux compilado no WSL a partir do código do Aylis desta máquina (só a primeira vez
/// demora; fica em `~/.aylis-wsl/target`). Ele prepara a linguagem no WSL (`setup`; o Rust do
/// WSL é o do rustup, já conferido acima) e publica no `out` (no Windows), com o pacote quando
/// `deb` (pedido do autor em 21/11: o `.deb` de um app no Windows e no Linux).
pub fn publish_with_tool_script(project: &Project, out: &Path, deb: bool) -> String {
    let setup = if project.language == aylis_project::Language::Rust {
        String::new()
    } else {
        format!(
            "echo '▶ Preparando {language} no Linux…'\n\"$T\" setup {lang} || echo \"aviso: o {language} do Linux não ficou pronto (veja acima)\"\n",
            language = project.language.name(),
            lang = project.language.folder(),
        )
    };
    format!(
        r#"set -e
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
if ! command -v cargo >/dev/null; then echo "o Rust não está instalado no Linux do WSL (Ferramentas → Verificar ambiente instala)" >&2; exit 3; fi
export CARGO_TARGET_DIR="$HOME/.aylis-wsl/target"
{tool}{setup}echo '▶ Publicando {name} (Linux)…'
"$T" publish {project} --out {out}{installer}
"#,
        tool = linux_tool(),
        name = project.name,
        project = quote(&to_linux(&project.path)),
        out = quote(&to_linux(out)),
        installer = if deb { " --installer" } else { "" },
    )
}

/// Roda um script no bash da distribuição, com cada linha nas mensagens. As linhas que
/// começam com `▶` viram passos (o "Compilando", o "Executando").
pub fn run(distro: &str, script: &str, control: &Control, sink: Sink, program_output: bool) -> Result<Option<i32>, String> {
    let mut command = wsl();
    command.args(["-d", distro, "--exec", "bash", "-lc", script]);
    let mut running = false;
    run_lines(&mut command, control, |line, is_err| {
        if let Some(step) = line.strip_prefix("▶ ") {
            running = step.starts_with("Executando");
            sink(Event::Step(step.to_string()));
            return;
        }
        let kind = match (running && program_output, is_err) {
            (true, false) => LineKind::Program,
            (true, true) => LineKind::ProgramError,
            _ => LineKind::Tool,
        };
        sink(Event::Line(line, kind));
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ubuntu_22_04_is_preferred_for_its_glibc() {
        let list = |l: &[&str]| l.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(pick(&list(&["Ubuntu", "Ubuntu-22.04"])).as_deref(), Some("Ubuntu-22.04"));
        assert_eq!(pick(&list(&["Ubuntu", "Debian"])).as_deref(), Some("Ubuntu"));
        assert_eq!(pick(&[]), None);
    }
}
