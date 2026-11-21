//! O que um app do Aylis precisa do sistema para compilar no Linux (o compilador C, que o Rust
//! usa como linker, e as bibliotecas de janela, fonte, OpenGL, áudio e controles), com o nome
//! do pacote em cada gerenciador: o `apt` (Debian, Ubuntu, Mint), o `dnf` (Fedora, RHEL), o
//! `zypper` (openSUSE) e o `pacman` (Arch). Antes só havia a lista do `apt`
//! (`crate::wsl::PACKAGES`), e no Fedora a verificação de ambiente mandava rodar um comando
//! que não existe lá (testado numa VM limpa em 20/11).

use std::path::Path;

/// O gerenciador de pacotes da distribuição.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Manager {
    Apt,
    Dnf,
    Zypper,
    Pacman,
}

impl Manager {
    /// O da distribuição pelo `/etc/os-release` (o `ID` e o `ID_LIKE`); sem ele, o primeiro
    /// programa que existir.
    pub fn detect() -> Option<Self> {
        let release = std::fs::read_to_string("/etc/os-release").unwrap_or_default();
        Self::from_os_release(&release).or_else(|| {
            [("apt-get", Self::Apt), ("dnf", Self::Dnf), ("zypper", Self::Zypper), ("pacman", Self::Pacman)]
                .into_iter()
                .find(|(program, _)| ["/usr/bin", "/bin"].iter().any(|d| Path::new(d).join(program).exists()))
                .map(|(_, m)| m)
        })
    }

    pub fn from_os_release(text: &str) -> Option<Self> {
        let field = |key: &str| {
            text.lines()
                .find_map(|l| l.strip_prefix(&format!("{key}=")))
                .map(|v| v.trim_matches('"').to_ascii_lowercase())
                .unwrap_or_default()
        };
        let ids = format!("{} {}", field("ID"), field("ID_LIKE"));
        let has = |names: &[&str]| ids.split_whitespace().any(|id| names.contains(&id));
        if has(&["debian", "ubuntu", "linuxmint", "pop", "elementary", "raspbian"]) {
            Some(Self::Apt)
        } else if has(&["fedora", "rhel", "centos", "rocky", "almalinux", "nobara"]) {
            Some(Self::Dnf)
        } else if has(&["opensuse", "suse", "opensuse-tumbleweed", "opensuse-leap", "sles"]) {
            Some(Self::Zypper)
        } else if has(&["arch", "manjaro", "endeavouros", "cachyos"]) {
            Some(Self::Pacman)
        } else {
            None
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Apt => "apt",
            Self::Dnf => "dnf",
            Self::Zypper => "zypper",
            Self::Pacman => "pacman",
        }
    }

    /// O comando que instala `packages` (como root).
    pub fn install_command(self, packages: &[&str]) -> String {
        let list = packages.join(" ");
        match self {
            Self::Apt => format!("apt-get update && apt-get install -y {list}"),
            Self::Dnf => format!("dnf install -y {list}"),
            Self::Zypper => format!("zypper --non-interactive install {list}"),
            Self::Pacman => format!("pacman -S --needed --noconfirm {list}"),
        }
    }
}

/// Uma dependência: o que se confere (o `pkg-config` dela ou um programa) e o pacote em cada
/// gerenciador (vazio: já vem com outro da lista).
pub struct Dep {
    pub label: &'static str,
    pub check: Check,
    /// apt, dnf, zypper, pacman.
    pub packages: [&'static str; 4],
}

pub enum Check {
    /// Um programa no PATH (o compilador C, o `pkg-config`).
    Program(&'static str),
    /// Um módulo do `pkg-config` (a biblioteca com os arquivos de desenvolvimento).
    PkgConfig(&'static str),
}

/// As mesmas de `crate::wsl::PACKAGES` (e da CI), na ordem.
pub const DEPS: [Dep; 15] = [
    Dep { label: "Compilador C (linker do Rust)", check: Check::Program("cc"), packages: ["build-essential", "gcc gcc-c++ make", "gcc gcc-c++ make", "base-devel"] },
    Dep { label: "pkg-config", check: Check::Program("pkg-config"), packages: ["pkg-config", "pkgconf-pkg-config", "pkg-config", "pkgconf"] },
    Dep { label: "X11", check: Check::PkgConfig("x11"), packages: ["libx11-dev", "libX11-devel", "libX11-devel", "libx11"] },
    Dep { label: "Xcursor", check: Check::PkgConfig("xcursor"), packages: ["libxcursor-dev", "libXcursor-devel", "libXcursor-devel", "libxcursor"] },
    Dep { label: "Xrandr", check: Check::PkgConfig("xrandr"), packages: ["libxrandr-dev", "libXrandr-devel", "libXrandr-devel", "libxrandr"] },
    Dep { label: "Xi", check: Check::PkgConfig("xi"), packages: ["libxi-dev", "libXi-devel", "libXi-devel", "libxi"] },
    Dep { label: "xkbcommon", check: Check::PkgConfig("xkbcommon"), packages: ["libxkbcommon-dev", "libxkbcommon-devel", "libxkbcommon-devel", "libxkbcommon"] },
    Dep { label: "xkbcommon-x11", check: Check::PkgConfig("xkbcommon-x11"), packages: ["libxkbcommon-x11-dev", "libxkbcommon-x11-devel", "libxkbcommon-x11-devel", "libxkbcommon-x11"] },
    Dep { label: "Wayland", check: Check::PkgConfig("wayland-client"), packages: ["libwayland-dev", "wayland-devel", "wayland-devel", "wayland"] },
    Dep { label: "Fontconfig", check: Check::PkgConfig("fontconfig"), packages: ["libfontconfig1-dev", "fontconfig-devel", "fontconfig-devel", "fontconfig"] },
    Dep { label: "FreeType", check: Check::PkgConfig("freetype2"), packages: ["libfreetype-dev", "freetype-devel", "freetype2-devel", "freetype2"] },
    Dep { label: "OpenGL", check: Check::PkgConfig("gl"), packages: ["libgl1-mesa-dev", "mesa-libGL-devel", "Mesa-libGL-devel", "mesa"] },
    Dep { label: "EGL", check: Check::PkgConfig("egl"), packages: ["libegl1-mesa-dev", "mesa-libEGL-devel", "Mesa-libEGL-devel", "mesa"] },
    Dep { label: "ALSA (áudio)", check: Check::PkgConfig("alsa"), packages: ["libasound2-dev", "alsa-lib-devel", "alsa-devel", "alsa-lib"] },
    Dep { label: "udev (controles de jogo)", check: Check::PkgConfig("libudev"), packages: ["libudev-dev", "systemd-devel", "systemd-devel", "systemd-libs"] },
];

impl Dep {
    pub fn package(&self, manager: Manager) -> &'static str {
        self.packages[manager as usize]
    }

    /// Está na máquina? (o `pkg-config` some junto com o que ele confere: sem ele, as
    /// bibliotecas contam como faltando).
    pub fn present(&self) -> bool {
        let quiet = |program: &str, args: &[&str]| {
            std::process::Command::new(program)
                .args(args)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .is_ok_and(|s| s.success())
        };
        match self.check {
            Check::Program(p) => quiet(p, &["--version"]),
            Check::PkgConfig(m) => quiet("pkg-config", &["--exists", m]),
        }
    }
}

/// As que faltam nesta máquina.
pub fn missing() -> Vec<&'static Dep> {
    DEPS.iter().filter(|d| !d.present()).collect()
}

/// Os pacotes de `deps` no gerenciador, sem repetir (o `mesa` do Arch serve ao GL e ao EGL).
pub fn packages_for(deps: &[&Dep], manager: Manager) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for d in deps {
        for p in d.package(manager).split_whitespace() {
            if !out.contains(&p) {
                out.push(p);
            }
        }
    }
    out
}

/// O script que roda `command` como root: pelo `sudo` sem senha, se a máquina permite (sem
/// tela, num servidor, na CI); senão pelo `pkexec`, que pede a senha numa janela do sistema.
/// O `command` não tem aspas simples (os comandos de `Manager::install_command`).
pub fn as_root(command: &str) -> String {
    format!("if sudo -n true 2>/dev/null; then sudo sh -c '{command}'; else pkexec sh -c '{command}'; fi")
}

/// O comando para o usuário rodar num terminal (com o `sudo`).
pub fn sudo_command(deps: &[&Dep], manager: Manager) -> String {
    let packages = packages_for(deps, manager);
    match manager {
        Manager::Apt => format!("sudo apt-get update && sudo apt-get install -y {}", packages.join(" ")),
        _ => format!("sudo {}", manager.install_command(&packages)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_distribution_picks_the_manager() {
        assert_eq!(Manager::from_os_release("NAME=\"Fedora Linux\"\nID=fedora\n"), Some(Manager::Dnf));
        assert_eq!(Manager::from_os_release("ID=linuxmint\nID_LIKE=\"ubuntu debian\"\n"), Some(Manager::Apt));
        assert_eq!(Manager::from_os_release("ID=\"opensuse-tumbleweed\"\nID_LIKE=\"opensuse suse\"\n"), Some(Manager::Zypper));
        assert_eq!(Manager::from_os_release("ID=manjaro\nID_LIKE=arch\n"), Some(Manager::Pacman));
        assert_eq!(Manager::from_os_release("ID=nixos\n"), None);
    }

    /// A lista do `apt` é a mesma de `crate::wsl::PACKAGES` (o `curl` à parte: ele vem em
    /// quase toda distribuição e o `.deb` o declara).
    #[test]
    fn the_apt_names_are_the_wsl_list() {
        let apt = packages_for(&DEPS.iter().collect::<Vec<_>>(), Manager::Apt);
        let wsl: Vec<&str> = crate::wsl::PACKAGES.iter().copied().filter(|p| *p != "curl").collect();
        assert_eq!(apt, wsl);
    }

    #[test]
    fn the_fedora_command_has_the_fedora_names() {
        let deps: Vec<&Dep> = DEPS.iter().collect();
        let cmd = sudo_command(&deps, Manager::Dnf);
        assert!(cmd.starts_with("sudo dnf install -y gcc gcc-c++ make pkgconf-pkg-config libX11-devel"), "{cmd}");
        assert!(cmd.contains("fontconfig-devel") && cmd.contains("alsa-lib-devel") && !cmd.contains("-dev "));
        // O `mesa` do Arch aparece uma vez só.
        assert_eq!(packages_for(&deps, Manager::Pacman).iter().filter(|p| **p == "mesa").count(), 1);
    }
}
