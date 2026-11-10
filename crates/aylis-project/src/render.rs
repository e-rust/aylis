//! Renderizador por plataforma: o que desenha o app em cada sistema (Direct2D no Windows,
//! Skia na GPU ou na CPU). A escolha vira features do crate `aylis` no `Cargo.toml` (o que
//! não for usado nem entra no executável) e é aplicada quando o app abre.
//!
//! Os números mostrados ao escolher foram medidos em 25/10/2026 (docs/renderizadores.md).

use serde::{Deserialize, Serialize};

/// O que desenha numa plataforma.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RenderChoice {
    /// Skia na GPU (OpenGL); sem OpenGL, Skia na CPU.
    #[default]
    Auto,
    /// Direct2D + DirectWrite (só Windows): sem Skia no executável.
    D2d,
    /// Skia na GPU (OpenGL).
    Gpu,
    /// Skia na CPU.
    Cpu,
}

impl RenderChoice {
    pub const ALL: [RenderChoice; 4] = [RenderChoice::D2d, RenderChoice::Gpu, RenderChoice::Cpu, RenderChoice::Auto];

    pub fn name(self) -> &'static str {
        match self {
            RenderChoice::Auto => "auto",
            RenderChoice::D2d => "d2d",
            RenderChoice::Gpu => "gpu",
            RenderChoice::Cpu => "cpu",
        }
    }

    /// Nome para a tela.
    pub fn title(self) -> &'static str {
        match self {
            RenderChoice::Auto => "Automático (Skia GPU, senão CPU)",
            RenderChoice::D2d => "Direct2D",
            RenderChoice::Gpu => "Skia GPU (OpenGL)",
            RenderChoice::Cpu => "Skia CPU",
        }
    }

    /// As features do crate `aylis` que esta escolha precisa.
    pub fn features(self) -> &'static [&'static str] {
        match self {
            RenderChoice::Auto => &["skia-gpu", "skia-cpu"],
            RenderChoice::D2d => &["d2d"],
            RenderChoice::Gpu => &["skia-gpu"],
            RenderChoice::Cpu => &["skia-cpu"],
        }
    }

    /// O que esperar (medido no Windows: tamanho do executável, tempo de um quadro do
    /// DataGrid e memória do processo).
    pub fn summary(self) -> &'static str {
        match self {
            RenderChoice::D2d => "executável ~2,2 MB · DataGrid 0,57 ms por quadro · memória ~45–70 MB. O mais leve em disco e o mais rápido em telas de sistema.",
            RenderChoice::Gpu => "executável ~19,3 MB · DataGrid 1,07 ms por quadro · memória ~300–570 MB (driver OpenGL). O mais rápido em telas muito carregadas de gráficos.",
            RenderChoice::Cpu => "executável ~19,3 MB · DataGrid 2,25 ms por quadro · memória ~12–16 MB. Não depende de GPU; o mais econômico em memória.",
            RenderChoice::Auto => "executável ~19,3 MB · usa a GPU (OpenGL) e cai para a CPU se não houver driver.",
        }
    }
}

/// As plataformas com escolha de renderizador (no navegador é sempre o Canvas 2D).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    Windows,
    Linux,
    MacOs,
    Android,
    Ios,
}

impl Platform {
    pub const ALL: [Platform; 5] = [Platform::Windows, Platform::Linux, Platform::MacOs, Platform::Android, Platform::Ios];

    pub fn title(self) -> &'static str {
        match self {
            Platform::Windows => "Windows",
            Platform::Linux => "Linux",
            Platform::MacOs => "macOS",
            Platform::Android => "Android",
            Platform::Ios => "iOS",
        }
    }

    /// A condição do Cargo (`[target.'cfg(...)'.dependencies]`).
    pub fn cfg(self) -> &'static str {
        match self {
            Platform::Windows => "cfg(windows)",
            Platform::Linux => "cfg(target_os = \"linux\")",
            Platform::MacOs => "cfg(target_os = \"macos\")",
            Platform::Android => "cfg(target_os = \"android\")",
            Platform::Ios => "cfg(target_os = \"ios\")",
        }
    }

    /// As escolhas que existem nesta plataforma (a primeira é a recomendada).
    pub fn choices(self) -> &'static [RenderChoice] {
        match self {
            Platform::Windows => &[RenderChoice::D2d, RenderChoice::Gpu, RenderChoice::Cpu, RenderChoice::Auto],
            Platform::Linux | Platform::MacOs | Platform::Android => &[RenderChoice::Auto, RenderChoice::Gpu, RenderChoice::Cpu],
            // No iOS ainda não há GPU (o glutin não tem OpenGL lá).
            Platform::Ios => &[RenderChoice::Cpu],
        }
    }

    /// A plataforma em que este código está rodando.
    pub fn current() -> Option<Platform> {
        if cfg!(windows) {
            Some(Platform::Windows)
        } else if cfg!(target_os = "android") {
            Some(Platform::Android)
        } else if cfg!(target_os = "ios") {
            Some(Platform::Ios)
        } else if cfg!(target_os = "macos") {
            Some(Platform::MacOs)
        } else if cfg!(target_os = "linux") {
            Some(Platform::Linux)
        } else {
            None
        }
    }
}

/// O renderizador de cada plataforma (no `.aylisproj`, `"render"`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderOptions {
    #[serde(default = "d2d")]
    pub windows: RenderChoice,
    #[serde(default)]
    pub linux: RenderChoice,
    #[serde(default)]
    pub macos: RenderChoice,
    #[serde(default)]
    pub android: RenderChoice,
    #[serde(default = "cpu")]
    pub ios: RenderChoice,
}

fn d2d() -> RenderChoice {
    RenderChoice::D2d
}

fn cpu() -> RenderChoice {
    RenderChoice::Cpu
}

impl Default for RenderOptions {
    /// O recomendado: Direct2D no Windows, Skia automático nas outras.
    fn default() -> Self {
        Self { windows: RenderChoice::D2d, linux: RenderChoice::Auto, macos: RenderChoice::Auto, android: RenderChoice::Auto, ios: RenderChoice::Cpu }
    }
}

impl RenderOptions {
    /// Como os projetos eram antes da escolha existir: Skia em todo lugar.
    pub fn legacy() -> Self {
        Self { windows: RenderChoice::Auto, ..Self::default() }
    }

    pub fn get(&self, platform: Platform) -> RenderChoice {
        match platform {
            Platform::Windows => self.windows,
            Platform::Linux => self.linux,
            Platform::MacOs => self.macos,
            Platform::Android => self.android,
            Platform::Ios => self.ios,
        }
    }

    /// Troca a escolha de uma plataforma (uma escolha que não existe nela é ignorada).
    pub fn set(&mut self, platform: Platform, choice: RenderChoice) {
        if !platform.choices().contains(&choice) {
            return;
        }
        match platform {
            Platform::Windows => self.windows = choice,
            Platform::Linux => self.linux = choice,
            Platform::MacOs => self.macos = choice,
            Platform::Android => self.android = choice,
            Platform::Ios => self.ios = choice,
        }
    }

    /// A escolha para a plataforma em que o app está rodando.
    pub fn current(&self) -> Option<RenderChoice> {
        Platform::current().map(|p| self.get(p))
    }
}

/// Quem mede e desenha o texto em cada plataforma (o `"text"` do `.aylisproj`, item 10):
/// `native` (cada renderizador, como sempre), `layout` (o núcleo mede e quebra, o
/// renderizador desenha cada linha) ou `core` (o núcleo também desenha as letras). Sem o campo,
/// um projeto de antes fica em `native`; os novos vêm com `core` no desktop e no celular e
/// `layout` na web.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TextOptions {
    #[serde(default = "core")]
    pub windows: TextChoice,
    #[serde(default = "core")]
    pub linux: TextChoice,
    #[serde(default = "core")]
    pub macos: TextChoice,
    #[serde(default = "core")]
    pub android: TextChoice,
    #[serde(default = "core")]
    pub ios: TextChoice,
    #[serde(default = "layout")]
    pub web: TextChoice,
}

/// Um modo de texto (veja [`TextOptions`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextChoice {
    #[default]
    Native,
    Layout,
    Core,
}

fn core() -> TextChoice {
    TextChoice::Core
}

fn layout() -> TextChoice {
    TextChoice::Layout
}

impl Default for TextOptions {
    /// O dos projetos novos: o núcleo desenha no desktop e no celular, e na web só mede.
    fn default() -> Self {
        Self { windows: TextChoice::Core, linux: TextChoice::Core, macos: TextChoice::Core, android: TextChoice::Core, ios: TextChoice::Core, web: TextChoice::Layout }
    }
}

impl TextOptions {
    /// Como os projetos eram antes da escolha existir: cada renderizador mede e desenha.
    pub fn legacy() -> Self {
        let n = TextChoice::Native;
        Self { windows: n, linux: n, macos: n, android: n, ios: n, web: n }
    }

    /// A escolha da plataforma `platform` (`None`: a web).
    pub fn get(&self, platform: Option<Platform>) -> TextChoice {
        match platform {
            Some(Platform::Windows) => self.windows,
            Some(Platform::Linux) => self.linux,
            Some(Platform::MacOs) => self.macos,
            Some(Platform::Android) => self.android,
            Some(Platform::Ios) => self.ios,
            None => self.web,
        }
    }

    pub fn set(&mut self, platform: Option<Platform>, choice: TextChoice) {
        match platform {
            Some(Platform::Windows) => self.windows = choice,
            Some(Platform::Linux) => self.linux = choice,
            Some(Platform::MacOs) => self.macos = choice,
            Some(Platform::Android) => self.android = choice,
            Some(Platform::Ios) => self.ios = choice,
            None => self.web = choice,
        }
    }

    /// A escolha para onde o app está rodando (a web quando não é nenhuma das plataformas).
    pub fn current(&self) -> TextChoice {
        self.get(Platform::current())
    }
}
