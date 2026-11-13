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

/// A distribuição usada: `AYLIS_WSL_DISTRO`, senão a primeira (o Ubuntu, normalmente).
pub fn distro() -> Option<String> {
    std::env::var("AYLIS_WSL_DISTRO").ok().filter(|d| !d.is_empty()).or_else(|| distros().into_iter().next())
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
/// pasta do código em Rust) e o `Cargo.toml` convertido; termina na pasta do crate.
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

/// O script da publicação: compila otimizado, copia o executável para `out` (no Windows) e,
/// com `deb`, monta um pacote `.deb` (instala em `/opt/<app>`, com atalho no menu).
pub fn publish_script(project: &Project, out: &Path, app_file: &str, deb: bool) -> String {
    let package = project.package_name();
    let out_l = quote(&to_linux(out));
    let mut s = prepare(project);
    s.push_str(&format!("echo '▶ Compilando {} (Linux, otimizado)…'\n", project.name));
    let bin = project.bin_name();
    s.push_str(&format!("cargo build --release --bin {bin}\n"));
    s.push_str(&format!("mkdir -p {out_l}\ncp \"$CARGO_TARGET_DIR/release/{bin}\" {out_l}/{}\n", quote(app_file)));
    if deb {
        let id = package.replace('_', "-");
        let version = project_version(project);
        s.push_str("echo '▶ Montando o pacote .deb…'\n");
        s.push_str(&format!(
            r#"D="$HOME/.aylis-wsl/deb/{id}"
rm -rf "$D"; mkdir -p "$D/DEBIAN" "$D/opt/{id}" "$D/usr/share/applications" "$D/usr/bin"
cp "$CARGO_TARGET_DIR/release/{package}" "$D/opt/{id}/{id}"
ln -s /opt/{id}/{id} "$D/usr/bin/{id}"
cat > "$D/DEBIAN/control" <<CONTROL
Package: {id}
Version: {version}
Architecture: amd64
Maintainer: {maintainer}
Description: {app}
Depends: libc6, libfontconfig1, libxkbcommon0, libgl1, libegl1
CONTROL
cat > "$D/usr/share/applications/{id}.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Name={app}
Exec=/opt/{id}/{id}
Terminal=false
Categories=Utility;
DESKTOP
dpkg-deb --build --root-owner-group "$D" {out_l}/{id}_{version}_amd64.deb >/dev/null
echo "pacote: {id}_{version}_amd64.deb"
"#,
            app = project.app_name(),
            maintainer = if project.publish.publisher.is_empty() { project.app_name().to_string() } else { project.publish.publisher.clone() },
        ));
    }
    s
}

/// A versão do pacote (a do `Cargo.toml` do projeto).
fn project_version(project: &Project) -> String {
    std::fs::read_to_string(project.code_dir().join("Cargo.toml"))
        .ok()
        .and_then(|t| t.lines().find_map(|l| l.trim().strip_prefix("version = \"").and_then(|v| v.strip_suffix('"')).map(str::to_string)))
        .unwrap_or_else(|| "1.0.0".into())
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
