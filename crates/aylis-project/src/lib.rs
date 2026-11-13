//! # aylis-project
//!
//! Um projeto do aylis é uma pasta (como um projeto do Delphi):
//!
//! ```text
//! Vendas/
//!   Vendas.aylisproj      índice: nome, linguagem, form principal, forms, tema, idioma
//!   forms/               um .json por form (o .fmx do aylis)
//!   assets/              images/, audio/, video/, fonts/
//!   themes/              temas próprios (opcional)
//!   src/                 código dos eventos (Rust: Cargo.toml na raiz; Python: main.py)
//! ```
//!
//! Este crate lê e grava o `.aylisproj`, cria projetos novos ([`create`]), instala no `Ui`
//! a fábrica de forms e o acesso aos arquivos do projeto ([`install`]) e gera o código dos
//! eventos ([`codegen`]). Abrir a janela fica com o crate `aylis` (`aylis::Application`).

pub mod assets;
pub mod bindings;
pub mod check;
pub mod codegen;
pub mod hostlang;
pub mod i18n;
pub mod new_project;
pub mod packages;
pub mod render;
pub mod symbols;
pub mod sync;

pub use render::{Platform, RenderChoice, RenderOptions, TextChoice, TextOptions};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use aylis_core::platform::{FormFactory, ResourceService};
use aylis_core::{ComponentId, Theme, Ui};
use serde::{Deserialize, Serialize};

/// Versão do formato do `.aylisproj`.
pub const FORMAT_VERSION: u32 = 1;

/// O que dizer de um projeto de um aylis anterior (a IDE oferece rodar o migrador).
pub const MIGRATE_HINT: &str = "o projeto é de uma versão anterior do Aylis: rode o aylis-migrate para converter";

/// Extensão do arquivo de projeto.
pub const EXTENSION: &str = "aylisproj";
/// A fonte embutida nos projetos novos (do catálogo offline em `resources/fonts/`).
pub const DEFAULT_FONT: &str = "Inter";

/// Linguagem do código dos eventos. Pode ser trocada a qualquer momento: os forms não
/// mudam; muda o código gerado e o jeito de executar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    Rust,
    Python,
    /// Node 22.6+ ou Bun.
    TypeScript,
    #[serde(rename = "csharp")]
    CSharp,
    Java,
    Go,
    Cpp,
    /// Free Pascal (o compilador do Lazarus).
    Pascal,
}

impl Language {
    pub const ALL: [Language; 8] =
        [Language::Rust, Language::Python, Language::TypeScript, Language::CSharp, Language::Java, Language::Go, Language::Cpp, Language::Pascal];

    pub fn name(self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::Python => "Python",
            Language::TypeScript => "TypeScript",
            Language::CSharp => "C#",
            Language::Java => "Java",
            Language::Go => "Go",
            Language::Cpp => "C++",
            Language::Pascal => "Free Pascal",
        }
    }

    /// Pelo nome (`"C#"`, `"Free Pascal"`...).
    pub fn from_name(name: &str) -> Option<Language> {
        Self::ALL.into_iter().find(|l| l.name().eq_ignore_ascii_case(name))
    }

    /// O backend roda pelo aylis-host, num processo à parte (todas menos Rust).
    pub fn uses_host(self) -> bool {
        self != Language::Rust
    }

    /// A pasta do código desta linguagem no projeto (`rust`, `python`, `csharp`...): cada
    /// linguagem tem a sua, e o visual (forms, temas, assets) é o mesmo para todas.
    pub fn folder(self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::TypeScript => "typescript",
            Language::CSharp => "csharp",
            Language::Java => "java",
            Language::Go => "go",
            Language::Cpp => "cpp",
            Language::Pascal => "pascal",
        }
    }
}

/// O que era código na raiz do projeto no layout antigo (de qualquer linguagem).
const LEGACY_CODE: [&str; 12] = ["src", "tests", "Cargo.toml", "Cargo.lock", "build.rs", "App.csproj", "go.mod", "go.sum", "main.go", "package.json", "tsconfig.json", "CMakeLists.txt"];

/// Um form do projeto.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormEntry {
    /// Nome do form (o `name` da raiz do layout), como `frmClientes`.
    pub name: String,
    /// Arquivo do layout, relativo à pasta do projeto (`forms/clientes.json`).
    pub file: String,
    /// Criado quando o aplicativo abre (como a lista "Auto-create forms" do Delphi). Os
    /// outros são criados sob demanda com `ui.create_form("frmClientes")`.
    #[serde(default)]
    pub auto_create: bool,
    /// A raiz do layout é um `DataModule`. Anotado ao salvar o form (e ao abrir o projeto, se o
    /// arquivo mudou fora da IDE): quem pergunta não abre o JSON.
    #[serde(default, skip_serializing_if = "is_false")]
    pub data_module: bool,
    /// Os DataModules que o form declara em `"uses"`, anotados como o `data_module`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uses: Vec<String>,
    /// A hora da última alteração do arquivo do form (milissegundos desde 1970) quando o
    /// `data_module` e o `uses` foram anotados; outra hora no disco faz ler o form de novo. Fica
    /// em `.aylis/form-stamps.json` (fora do controle de versão), não no `.aylisproj`: gravada nele,
    /// mudava a cada salvar de form, e duas pessoas mexendo em forms diferentes conflitavam no
    /// `.aylisproj` (conferência de 08/11). Um `.aylisproj` de antes ainda é lido.
    #[serde(default, skip_serializing)]
    pub modified: Option<u64>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// A hora da última alteração de um arquivo, em milissegundos desde 1970.
pub fn modified_ms(path: &Path) -> Option<u64> {
    let time = std::fs::metadata(path).ok()?.modified().ok()?;
    Some(time.duration_since(std::time::UNIX_EPOCH).ok()?.as_millis() as u64)
}

/// O `.aylisproj`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "aylis_project")]
    pub format: u32,
    pub name: String,
    #[serde(default)]
    pub language: Language,
    /// Nome do form principal (fechá-lo encerra o aplicativo).
    pub main_form: String,
    pub forms: Vec<FormEntry>,
    /// O tema do projeto inteiro: `"light"`, `"dark"`, um tema de plataforma pelo nome
    /// (`"Material 3 Light"`, `"Windows 11 Dark"`...) ou um arquivo de tema
    /// (`themes/empresa.json`). Vazio = claro.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub theme: String,
    /// Fonte do projeto inteiro (família de `assets/fonts/` ou do sistema). Vazio = a do
    /// tema.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub font: String,
    /// As fontes do projeto por nome, como as cores: `"titulo": "Montserrat"`. Um componente
    /// com `font_family: "titulo"` (ou um estilo com `"family": "titulo"`) usa essa família;
    /// trocar aqui troca em todos.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub fonts: std::collections::BTreeMap<String, String>,
    /// As cores do projeto, por nome: `"brand": "#10B981"`, um valor por modo
    /// (`"painel": { "dark": "#0F0F0F", "light": "#FFFFFF" }`) ou outra cor com opacidade
    /// (`"brand_soft": "brand/15"`). Valem em qualquer propriedade de cor e nos estilos,
    /// em todo tema (veja `aylis_core::palette`).
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub colors: serde_json::Map<String, serde_json::Value>,
    /// Como a roda rola: `{ "wheel_step": 100, "smooth": true, "smooth_ms": 160 }` (pontos
    /// por dente, deslizar e a duração do deslize). Sem ele, 40 pontos e sem deslize.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll: Option<ScrollSettings>,
    /// O zoom do app: `{ "value": 1.25, "keys": true }` (o tamanho de tudo; com `keys`, Ctrl
    /// + `+`/`-`/`0` e Ctrl + roda mudam). Sem ele, 1 e sem teclas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zoom: Option<ZoomSettings>,
    /// O fuso do app, pelo nome IANA (`"America/Sao_Paulo"`): todos veem a hora da empresa,
    /// mesmo em Manaus. Vazio = o fuso do aparelho. Liga a feature `time-zones` do aylis (a
    /// tabela de fusos vai junto no app).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub time_zone: String,
    /// Features do crate `aylis` que o código do app usa por conta própria (`["media"]`: o
    /// áudio por `aylis::media` sem componente de mídia no form). Somam-se às que os forms
    /// pedem; sem isso, tirar o último componente de mídia tirava a feature do código.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<String>,
    /// Os limites de segurança: `{ "tree_depth": 256, "event_nesting": 64 }` (a profundidade
    /// da árvore de componentes e os eventos aninhados); passar deles é um erro. Sem ele,
    /// esses padrões.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<LimitSettings>,
    /// O cache de imagens prontas dos renderizadores: `{ "pixels_mb": 96, "files": 256,
    /// "layers_mb": 128, "auto_layers": true }`. O que o app solta sai sozinho; os limites só
    /// contam para o que ele ainda segura. Passar deles não erra a tela, só refaz (decodifica,
    /// reenvia à placa, repinta) a imagem usada há mais tempo quando ela voltar. As camadas
    /// automáticas (`auto_layers`) valem em todos os forms, salvo o `layer_cache` de um form.
    /// Sem ele, esses padrões (`aylis_core::ImageCacheLimits`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_cache: Option<ImageCacheSettings>,
    /// Onde fica o executável de desenvolvimento (a "pasta de saída" do Delphi): o Executar
    /// e a depuração compilam, copiam o executável para lá e rodam de lá (ao lado das DLLs
    /// e dos arquivos que o app precisa). Sem ela, roda do `target` do cargo.
    #[serde(default, skip_serializing_if = "OutputSettings::is_empty")]
    pub output: OutputSettings,
    /// Classes com o visual de outro tema: classe → nome do tema (`"Button": "iOS 26 Light"`).
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub theme_mix: std::collections::BTreeMap<String, String>,
    /// `"pt-BR"` ou `"en-US"`. Vazio = pt-BR.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub locale: String,
    /// Nome, versão, ícone e identificador do aplicativo publicado.
    #[serde(default, skip_serializing_if = "PublishOptions::is_empty")]
    pub publish: PublishOptions,
    /// O renderizador de cada plataforma. `None` (projetos de antes dessa opção) = Skia
    /// em todas, como era ([`RenderOptions::legacy`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render: Option<RenderOptions>,
    /// Quem mede e desenha o texto em cada plataforma (item 10); sem ele, `native` em todas
    /// ([`TextOptions::legacy`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<TextOptions>,
    /// A placa de vídeo quando há mais de uma: `alto-desempenho`, `economia` ou parte do
    /// nome dela. Vazio = a do sistema.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub gpu: String,
    /// Como o quadro chega à janela no Windows: vazio, a swap chain da janela (como os jogos:
    /// a menor latência em tela cheia), ou `composition` (DirectComposition, como os
    /// navegadores: sempre pelo compositor do Windows).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub present: String,
    /// De onde vem o Aylis no `Cargo.toml` (o relatório do Tabernáculo, G11/G03):
    /// `crates.io` (a versão publicada que a IDE usa; o padrão dos projetos novos), uma pasta
    /// (relativa ao projeto ou absoluta: um Aylis fixo, que o gerador não troca) ou vazio (a
    /// junção `.aylis/sdk` até o Aylis instalado nesta máquina, como os projetos antigos).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sdk: String,
    /// Onde a IDE espera o app em desenvolvimento (`host:porta`), embutido no app compilado
    /// com a feature `dev`. Vazio = `127.0.0.1:7342` (desktop, navegador local e o celular
    /// pelo `adb reverse`); o IP da máquina para um celular sem cabo.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub dev_address: String,
    /// As plataformas de destino (as chaves de [`new_project::TARGETS`]: `windows`, `android`...),
    /// escolhidas no assistente de novo projeto; vazio = todas.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<String>,
    /// O aparelho em que o designer mostra os forms e o Executar abre a janela, com a moldura
    /// (`"Pixel 8"`, `"Pixel 8;deitado"`; os de [`aylis_core::devices::DEVICES`]). Vazio: sem
    /// aparelho. O modelo Smartphone já vem com um (decidido pelo autor em 08/11).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub device: String,
    /// Onde o `.aylisproj` está (não vai para o arquivo).
    #[serde(skip)]
    pub path: PathBuf,
}

pub type Error = String;

/// Os limites de segurança (o `limits` do `.aylisproj`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitSettings {
    #[serde(default = "LimitSettings::default_tree_depth")]
    pub tree_depth: usize,
    #[serde(default = "LimitSettings::default_event_nesting")]
    pub event_nesting: usize,
}

impl LimitSettings {
    fn default_tree_depth() -> usize {
        aylis_core::MAX_TREE_DEPTH
    }

    fn default_event_nesting() -> usize {
        aylis_core::MAX_EVENT_NESTING
    }

    /// O que passa do máximo seguro para a pilha (para a IDE avisar); vazio se nada.
    pub fn warnings(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.tree_depth > aylis_core::SAFE_TREE_DEPTH {
            out.push(format!("limits.tree_depth = {} passa do máximo seguro ({}): a pilha pode estourar antes", self.tree_depth, aylis_core::SAFE_TREE_DEPTH));
        }
        if self.event_nesting > aylis_core::SAFE_EVENT_NESTING {
            out.push(format!("limits.event_nesting = {} passa do máximo seguro ({}): a pilha pode estourar antes", self.event_nesting, aylis_core::SAFE_EVENT_NESTING));
        }
        out
    }
}

/// O cache de imagens (o `image_cache` do `.aylisproj`); veja [`aylis_core::ImageCacheLimits`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageCacheSettings {
    /// Megabytes de imagens em pixels na placa (vídeo, câmera, prévias, blocos de desenho):
    /// precisa caber o que aparece junto na tela.
    #[serde(default = "ImageCacheSettings::default_pixels_mb")]
    pub pixels_mb: u32,
    /// Quantas imagens de arquivo (PNG, JPEG, SVG...) ficam prontas: precisa caber as que
    /// aparecem juntas numa tela (ícones, miniaturas, galeria).
    #[serde(default = "ImageCacheSettings::default_files")]
    pub files: u32,
    /// Megabytes das camadas guardadas (o `cache_layer` e as páginas de uma transição, como
    /// imagem): precisa caber as que andam juntas; `0`, nenhuma fica guardada.
    #[serde(default = "ImageCacheSettings::default_layers_mb")]
    pub layers_mb: u32,
    /// As camadas automáticas (as páginas de uma transição, o contêiner animando) em todos os
    /// forms; cada form pode trocar pela propriedade `layer_cache`.
    #[serde(default = "ImageCacheSettings::default_auto_layers")]
    pub auto_layers: bool,
}

impl ImageCacheSettings {
    fn default_pixels_mb() -> u32 {
        aylis_core::ImageCacheLimits::DEFAULT.pixels_mb
    }
    fn default_files() -> u32 {
        aylis_core::ImageCacheLimits::DEFAULT.files
    }
    fn default_layers_mb() -> u32 {
        aylis_core::ImageCacheLimits::DEFAULT.layers_mb
    }
    fn default_auto_layers() -> bool {
        true
    }

    pub fn limits(&self) -> aylis_core::ImageCacheLimits {
        aylis_core::ImageCacheLimits { pixels_mb: self.pixels_mb, files: self.files, layers_mb: self.layers_mb }
    }
}

impl Default for ImageCacheSettings {
    fn default() -> Self {
        let d = aylis_core::ImageCacheLimits::DEFAULT;
        Self { pixels_mb: d.pixels_mb, files: d.files, layers_mb: d.layers_mb, auto_layers: true }
    }
}

/// A rolagem pela roda do mouse (o `scroll` do `.aylisproj`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScrollSettings {
    #[serde(default = "ScrollSettings::default_step")]
    pub wheel_step: f32,
    #[serde(default)]
    pub smooth: bool,
    #[serde(default = "ScrollSettings::default_ms")]
    pub smooth_ms: f32,
}

impl ScrollSettings {
    fn default_step() -> f32 {
        aylis_core::WHEEL_LINE
    }
    fn default_ms() -> f32 {
        160.0
    }
}

/// A pasta de saída do executável de desenvolvimento (o `output` do `.aylisproj`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OutputSettings {
    /// A pasta, relativa ao projeto (`rust/exe`) ou absoluta. Vazia = o `target` do cargo.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub dir: String,
    /// O nome do executável lá, sem a extensão (`NoOBS`). Vazio = o do cargo.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// O binário (`[[bin]]`) que o Compilar, o Executar e o Publicar usam, num crate com
    /// vários ou com outro nome. Vazio = o primeiro `[[bin]]` do `Cargo.toml`, ou o pacote. O
    /// relatório do Tabernáculo, I08 e G13.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub bin: String,
    /// Projetos em Rust: quantos dias o que é só deste projeto (o código dele, os testes e os
    /// exemplos compilados) fica na pasta de compilação depois da última compilação. Sem ele,
    /// o padrão das Opções da IDE (`aylis_tools::cleanup`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleanup_days: Option<u32>,
    /// Encerrar o app, se ele já estiver aberto, antes de executar (pedido do autor em 08/11: o
    /// NoOBS da IDE terminava na hora com o instalado aberto). Sem isso, o Executar avisa e para.
    #[serde(default, skip_serializing_if = "is_false")]
    pub close_running: bool,
    /// Comandos antes da compilação (como os Build Events do Delphi). Só rodam em projeto
    /// confiado, e o "Confiar neste projeto?" mostra cada um.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub before_build: Vec<BuildCommand>,
    /// Comandos depois da compilação (com o executável pronto, em `AYLIS_OUTPUT_EXE`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub after_build: Vec<BuildCommand>,
}

impl OutputSettings {
    pub fn is_empty(&self) -> bool {
        self.dir.is_empty() && self.name.is_empty() && self.cleanup_days.is_none() && !self.close_running && self.before_build.is_empty() && self.after_build.is_empty()
    }
}

/// Um comando antes ou depois da compilação: o texto, rodado pelo `cmd` ou pelo PowerShell na
/// pasta do projeto.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BuildCommand {
    #[serde(default)]
    pub shell: BuildShell,
    pub command: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildShell {
    #[default]
    Cmd,
    PowerShell,
}

impl BuildShell {
    pub fn name(self) -> &'static str {
        match self {
            BuildShell::Cmd => "cmd",
            BuildShell::PowerShell => "PowerShell",
        }
    }
}

impl Project {
    /// Os comandos antes e depois da compilação, para mostrar (o "Confiar neste projeto?").
    pub fn build_commands(&self) -> Vec<String> {
        let list = |when: &str, cmds: &[BuildCommand]| cmds.iter().map(|c| format!("{when} ({}): {}", c.shell.name(), c.command)).collect::<Vec<_>>();
        let mut out = list("antes de compilar", &self.output.before_build);
        out.extend(list("depois de compilar", &self.output.after_build));
        out
    }
}

impl Project {
    /// Para onde vai o executável que o cargo gerou (`built`): a pasta de saída com o nome
    /// dela; `None` sem pasta de saída.
    pub fn output_exe(&self, built: &Path) -> Option<PathBuf> {
        if self.output.dir.is_empty() {
            return None;
        }
        let dir = Path::new(&self.output.dir);
        let dir = if dir.is_absolute() { dir.to_path_buf() } else { self.dir().join(dir) };
        let file = match &self.output.name {
            n if n.is_empty() => built.file_name()?.to_os_string(),
            n => match built.extension() {
                Some(ext) => format!("{n}.{}", ext.to_string_lossy()).into(),
                None => n.into(),
            },
        };
        Some(dir.join(file))
    }
}

/// O zoom do app (o `zoom` do `.aylisproj`).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoomSettings {
    #[serde(default = "ZoomSettings::default_value")]
    pub value: f32,
    #[serde(default)]
    pub keys: bool,
}

impl ZoomSettings {
    fn default_value() -> f32 {
        1.0
    }
}

/// As opções do manifesto do Android (Projeto → Android…). Zero ou vazio: o padrão do aylis.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AndroidOptions {
    /// O `versionCode` (0: tirado da versão, 1.2.3 → 10203).
    #[serde(skip_serializing_if = "is_zero")]
    pub version_code: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub min_sdk: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub target_sdk: u32,
    /// `portrait`, `landscape`, `sensorPortrait`...; vazio: gira com o aparelho.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub orientation: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub fullscreen: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub keep_screen_on: bool,
    /// Só para tablets (as telas grandes; fora dos celulares na loja).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub tablet_only: bool,
    /// Permissões além das que os componentes pedem (`android.permission.CALL_PHONE`).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
    /// `uses-feature`: nome e se é obrigatório (`android.hardware.nfc`).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<(String, bool)>,
    /// Links que abrem o app: `https://empresa.com/pedido` ou um esquema próprio (`meuapp://`).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deep_links: Vec<String>,
}

fn is_zero(n: &u32) -> bool {
    *n == 0
}

impl AndroidOptions {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

/// Como o aplicativo aparece quando publicado (Projeto → Opções).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PublishOptions {
    /// Nome mostrado ao usuário (título do ícone, nome no celular). Vazio = nome do projeto.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub app_name: String,
    /// Versão (`1.0.0`). Vazio = `1.0.0`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
    /// Ícone: um PNG do projeto (256 px ou mais; 1024 deixa nítido em todos) ou um SVG. A IDE
    /// gera o de cada plataforma ao publicar.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub icon: String,
    /// A cor de fundo do ícone adaptativo do Android (`#RRGGBB`); vazio = a da borda da imagem.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub icon_background: String,
    /// As opções do Android (Projeto → Android…).
    #[serde(default, skip_serializing_if = "AndroidOptions::is_default")]
    pub android: AndroidOptions,
    /// Identificador do aplicativo no Android e no iOS (`com.empresa.vendas`). Vazio =
    /// `dev.aylis.<nome>`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub identifier: String,
    /// Android: a chave de assinatura do desenvolvedor (um `.jks`/`.keystore`, relativo ao
    /// projeto) e o apelido dela. As senhas nunca ficam no projeto: vêm de
    /// `AYLIS_KEYSTORE_PASSWORD`/`AYLIS_KEY_PASSWORD` ou a IDE pergunta. Vazio: a chave de
    /// depuração (não serve para a Play Store).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub keystore: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key_alias: String,
    /// Windows: o fabricante no instalador (`.msi`). Vazio: o nome do aplicativo.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub publisher: String,
    /// As opções do Windows: o tipo de instalador e o que vai nele.
    #[serde(default, skip_serializing_if = "WindowsOptions::is_default")]
    pub windows: WindowsOptions,
    /// Projetos que não são em Rust, no celular e no navegador: o servidor onde o código
    /// roda (`wss://servidor:9443`, com `aylis-host --bridge` lá). O app leva os forms e
    /// conecta nele.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub backend_url: String,
}

impl PublishOptions {
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

/// O instalador do Windows (Publicar → Windows → instalador). Vazio: o `.msi` de sempre, só
/// com o executável.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WindowsOptions {
    /// `msi` (o padrão: WiX, por máquina, em Arquivos de Programas), `nsis` (um `.exe` do NSIS)
    /// ou `inno` (um `.exe` do Inno Setup). Os dois `.exe` instalam por usuário, sem pedir
    /// administrador, com as opções abaixo.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub installer: String,
    /// Arquivos que vão ao lado do executável (DLLs de terceiros), relativos ao projeto;
    /// `pasta/*.dll` pega os de uma pasta. Valem para os dois instaladores e para a pasta
    /// publicada.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    /// NSIS e Inno: a pasta de instalação, com os marcadores `{localappdata}`, `{appdata}`,
    /// `{programfiles}` e `{documents}` (cada instalador troca pelo dele):
    /// `{localappdata}\Empresa\App`. Vazio: `{localappdata}\Programs\<app>`.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub install_dir: String,
    /// NSIS e Inno: a subpasta do executável dentro da de instalação (`bin`). Vazio: a própria.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub exe_dir: String,
    /// NSIS e Inno: oferecer "Iniciar com o Windows" (`on`: marcado; `off`: desmarcado; vazio:
    /// não oferece).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub autostart: String,
    /// Os argumentos do executável quando ele inicia com o Windows (`/autostart`).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub autostart_args: String,
    /// NSIS e Inno: oferecer o atalho na área de trabalho (`on`, `off` ou vazio, como o
    /// `autostart`).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub desktop_shortcut: String,
    /// NSIS e Inno: com o app aberto, pergunta e fecha antes de instalar e de desinstalar.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub close_running: bool,
    /// NSIS e Inno: abre o app no fim da instalação.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub run_after_install: bool,
    /// NSIS e Inno: ao desinstalar, pergunta se apaga também esta pasta de dados do app
    /// (`{localappdata}\App`, com os marcadores do `install_dir`). Vazio: não pergunta.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub data_dir: String,
}

/// O tipo de instalador do Windows ([`WindowsOptions::installer`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowsInstaller {
    Msi,
    Nsis,
    Inno,
}

impl WindowsOptions {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    /// O tipo pedido (desconhecido ou vazio: o `.msi`).
    pub fn kind(&self) -> WindowsInstaller {
        match self.installer.to_ascii_lowercase().as_str() {
            "nsis" => WindowsInstaller::Nsis,
            "inno" | "innosetup" | "inno_setup" => WindowsInstaller::Inno,
            _ => WindowsInstaller::Msi,
        }
    }
}

impl Project {
    /// As cores do projeto (`colors`) na forma do `Ui` ([`Ui::set_palette`]).
    pub fn palette(&self) -> aylis_core::palette::Palette {
        use aylis_core::palette::PaletteColor;
        self.colors
            .iter()
            .filter_map(|(name, v)| {
                let color = match v {
                    serde_json::Value::String(s) => PaletteColor::same(s.clone()),
                    serde_json::Value::Object(m) => {
                        let dark = m.get("dark").and_then(|v| v.as_str());
                        let light = m.get("light").and_then(|v| v.as_str());
                        let (dark, light) = (dark.or(light)?, light.or(dark)?);
                        PaletteColor::by_mode(dark, light)
                    }
                    _ => return None,
                };
                Some((name.clone(), color))
            })
            .collect()
    }

    /// O renderizador de cada plataforma (o de antes, Skia, para projetos sem a opção).
    pub fn render(&self) -> RenderOptions {
        self.render.unwrap_or_else(RenderOptions::legacy)
    }

    /// Os modos de texto do projeto (veja [`TextOptions`]).
    pub fn text(&self) -> TextOptions {
        self.text.unwrap_or_else(TextOptions::legacy)
    }

    /// Nome do aplicativo publicado.
    pub fn app_name(&self) -> &str {
        if self.publish.app_name.is_empty() { &self.name } else { &self.publish.app_name }
    }

    /// Versão do aplicativo publicado.
    pub fn version(&self) -> &str {
        if self.publish.version.is_empty() { "1.0.0" } else { &self.publish.version }
    }

    /// Identificador (`com.empresa.app`) do aplicativo publicado.
    pub fn identifier(&self) -> String {
        if self.publish.identifier.is_empty() {
            let clean: String = self.name.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect();
            format!("dev.aylis.{}", if clean.is_empty() { "app".to_string() } else { clean })
        } else {
            self.publish.identifier.clone()
        }
    }

    /// Nome do pacote do Cargo: o do `Cargo.toml` do projeto (um projeto pode ter escolhido
    /// outro, como o `noobs` do NoOBS); sem ele, o que sai do nome (`Vendas Web` →
    /// `vendas_web`).
    pub fn package_name(&self) -> String {
        std::fs::read_to_string(self.code_dir_of(Language::Rust).join("Cargo.toml"))
            .ok()
            .and_then(|text| cargo_package_name(&text))
            .unwrap_or_else(|| codegen::package_name(&self.name))
    }

    /// O executável do desktop: o primeiro `[[bin]]` do `Cargo.toml` (o pacote pode ter outro
    /// nome, como o `rtsplayer-app` com o executável `rtsplayer`); sem `[[bin]]` com nome, o do
    /// pacote.
    pub fn bin_name(&self) -> String {
        if !self.output.bin.trim().is_empty() {
            return self.output.bin.trim().to_string();
        }
        std::fs::read_to_string(self.code_dir_of(Language::Rust).join("Cargo.toml"))
            .ok()
            .and_then(|text| cargo_section_name(&text, "[[bin]]"))
            .unwrap_or_else(|| self.package_name())
    }
}

/// O `name` da seção `[package]` de um `Cargo.toml`.
fn cargo_package_name(text: &str) -> Option<String> {
    cargo_section_name(text, "[package]")
}

/// O `name` da primeira seção `section` (`[package]`, `[[bin]]`) de um `Cargo.toml`.
fn cargo_section_name(text: &str, section: &str) -> Option<String> {
    let mut in_package = false;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line == section;
            continue;
        }
        if !in_package {
            continue;
        }
        if let Some(value) = line.strip_prefix("name").map(str::trim_start).and_then(|r| r.strip_prefix('=')) {
            let value = value.trim().trim_matches('"').trim_matches('\'');
            return (!value.is_empty()).then(|| value.to_string());
        }
    }
    None
}

impl Project {
    /// Lê um `.aylisproj`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path).map_err(|e| format!("não consegui ler {}: {e}", path.display()))?;
        // Sem o BOM que alguns editores (o Bloco de Notas) põem no começo.
        let mut project: Project = serde_json::from_str(text.trim_start_matches('\u{feff}')).map_err(|e| format!("{}: {e}", path.display()))?;
        // Só o próprio formato (sem arrastar o antigo): outro passa pelo aylis-migrate.
        if project.format > FORMAT_VERSION {
            return Err(format!("{}: formato {} é mais novo que este aylis (que lê o {FORMAT_VERSION}): atualize o aylis", path.display(), project.format));
        }
        if project.format < FORMAT_VERSION {
            return Err(format!("{}: {MIGRATE_HINT} (formato {} → {FORMAT_VERSION})", path.display(), project.format));
        }
        project.path = path.to_path_buf();
        // Os forms ficam dentro da pasta do projeto: um `.aylisproj` de fora com
        // `../../algum/lugar.json` faria a IDE gravar fora dela ao salvar.
        if let Some(bad) = project.forms.iter().find(|f| !inside_project(&f.file)) {
            return Err(format!("{}: o form {} aponta para fora do projeto ({})", path.display(), bad.name, bad.file));
        }
        // O que o `.aylisproj` anota de cada form (DataModule, `uses`): os que mudaram fora da IDE
        // são lidos de novo (a IDE grava ao salvar). As horas anotadas ficam fora do `.aylisproj`.
        let stamps = project.load_stamps();
        for f in &mut project.forms {
            if let Some(&ms) = stamps.get(&f.name) {
                f.modified = Some(ms);
            }
        }
        project.refresh_forms();
        Ok(project)
    }

    /// Grava o `.aylisproj` (em `self.path`) e as horas dos forms (`.aylis/form-stamps.json`).
    pub fn save(&self) -> Result<(), Error> {
        let text = serde_json::to_string_pretty(self).map_err(|e| e.to_string())? + "\n";
        // Igual ao que está no disco: não regrava (o build salvava o projeto a cada vez; I06 do
        // Tabernáculo).
        if std::fs::read_to_string(&self.path).ok().as_deref() != Some(text.as_str()) {
            std::fs::write(&self.path, text).map_err(|e| format!("não consegui gravar {}: {e}", self.path.display()))?;
        }
        self.save_stamps();
        Ok(())
    }

    /// O arquivo das horas dos forms (na pasta `.aylis`, fora do controle de versão).
    fn stamps_file(&self) -> PathBuf {
        self.dir().join(".aylis").join("form-stamps.json")
    }

    fn load_stamps(&self) -> std::collections::HashMap<String, u64> {
        std::fs::read_to_string(self.stamps_file()).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
    }

    /// Grava as horas dos forms (só quando mudaram), com a regra no `.aylis/.gitignore`.
    fn save_stamps(&self) {
        let stamps: std::collections::BTreeMap<&str, u64> = self.forms.iter().filter_map(|f| Some((f.name.as_str(), f.modified?))).collect();
        let file = self.stamps_file();
        let text = serde_json::to_string_pretty(&stamps).unwrap_or_default() + "\n";
        if std::fs::read_to_string(&file).is_ok_and(|old| old == text) {
            return;
        }
        let Some(dir) = file.parent() else { return };
        if stamps.is_empty() && !dir.is_dir() {
            return;
        }
        local_dir_ignored(dir);
        let _ = std::fs::write(&file, text);
    }

    /// A pasta do projeto.
    pub fn dir(&self) -> PathBuf {
        self.path.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf)
    }

    /// A pasta do código da linguagem ativa (`<projeto>/rust`). As outras linguagens que o
    /// projeto já teve continuam nas pastas delas, intactas.
    pub fn code_dir(&self) -> PathBuf {
        self.code_dir_of(self.language)
    }

    /// A pasta do código de uma linguagem (`<projeto>/python`), exista ou não.
    pub fn code_dir_of(&self, language: Language) -> PathBuf {
        self.dir().join(language.folder())
    }

    /// As linguagens que têm código no projeto (uma pasta cada), na ordem de [`Language::ALL`].
    pub fn code_languages(&self) -> Vec<Language> {
        Language::ALL.into_iter().filter(|l| self.code_dir_of(*l).is_dir()).collect()
    }

    /// O mesmo projeto com outra linguagem ativa (só em memória).
    pub fn with_language(&self, language: Language) -> Project {
        Project { language, ..self.clone() }
    }

    /// Projeto do layout antigo (o código na raiz, ao lado dos forms): move o código para a
    /// pasta da linguagem dele. Devolve se moveu.
    pub fn migrate_layout(&self) -> Result<bool, Error> {
        let dir = self.dir();
        let target = self.code_dir();
        if target.exists() || !dir.join("src").is_dir() {
            return Ok(false);
        }
        std::fs::create_dir_all(&target).map_err(|e| format!("não consegui criar {}: {e}", target.display()))?;
        for item in LEGACY_CODE {
            let from = dir.join(item);
            if from.exists() {
                std::fs::rename(&from, target.join(item)).map_err(|e| format!("não consegui mover {} para {}: {e}", from.display(), target.display()))?;
            }
        }
        Ok(true)
    }

    /// Caminho completo de um arquivo do projeto (`forms/clientes.json`).
    pub fn file(&self, relative: &str) -> PathBuf {
        self.dir().join(relative)
    }

    pub fn form(&self, name: &str) -> Option<&FormEntry> {
        self.forms.iter().find(|f| f.name.eq_ignore_ascii_case(name))
    }

    /// Lê o layout de um form.
    pub fn read_form(&self, name: &str) -> Result<String, Error> {
        let entry = self.form(name).ok_or_else(|| format!("o projeto não tem o form '{name}'"))?;
        let path = self.file(&entry.file);
        std::fs::read_to_string(&path).map_err(|e| format!("não consegui ler {}: {e}", path.display()))
    }

    /// Move ou renomeia o arquivo do form `name` para `new_file` (`forms/painel/inicio.json`):
    /// o JSON e as variantes dele, e o fonte do form em cada linguagem do projeto (o módulo sai
    /// do nome do arquivo: renomear muda o módulo; mudar só de pasta, não). Grava o `.aylisproj`.
    /// Antes, só o "Salvar como…" com a janela do sistema fazia isso, fora do alcance do MCP
    /// (relatório do InsightDB, 11/11).
    pub fn move_form(&mut self, name: &str, new_file: &str) -> Result<(), Error> {
        let index = self.forms.iter().position(|f| f.name.eq_ignore_ascii_case(name)).ok_or_else(|| format!("o projeto não tem o form '{name}'"))?;
        let new_file = new_file.trim().replace('\\', "/");
        let path = Path::new(&new_file);
        let normal = path.components().all(|c| matches!(c, std::path::Component::Normal(_)));
        if !normal || !new_file.starts_with("forms/") || !new_file.ends_with(".json") {
            return Err(format!("'{new_file}': use um caminho dentro de forms/, terminado em .json (ex.: forms/painel/inicio.json)"));
        }
        let old = self.forms[index].clone();
        if old.file == new_file {
            return Ok(());
        }
        if self.file(&new_file).exists() || self.forms.iter().any(|f| f.file.eq_ignore_ascii_case(&new_file)) {
            return Err(format!("{new_file} já existe"));
        }
        let new_module = codegen::module_name(&new_file);
        if self.forms.iter().enumerate().any(|(i, f)| i != index && codegen::module_name(&f.file) == new_module) {
            return Err(format!("outro form já usa o módulo de código '{new_module}' (o nome do arquivo): escolha outro nome"));
        }
        let moved = FormEntry { file: new_file.clone(), ..old.clone() };
        // O fonte do form em cada linguagem (antes de mexer no JSON: se algum já existe, nada muda).
        let mut sources = Vec::new();
        for language in self.code_languages() {
            let project = self.with_language(language);
            let (from, to) = (codegen::entry_source(&project, &old), codegen::entry_source(&project, &moved));
            if from != to && from.is_file() {
                if to.exists() {
                    return Err(format!("{} já existe", to.display()));
                }
                sources.push((from, to));
            }
        }
        let mut files = vec![(old.file.clone(), new_file.clone())];
        files.extend(VARIANTS.iter().map(|v| (variant_file(&old.file, v), variant_file(&new_file, v))).filter(|(from, _)| self.file(from).is_file()));
        for (from, to) in files.iter().map(|(a, b)| (self.file(a), self.file(b))).chain(sources) {
            if let Some(dir) = to.parent() {
                std::fs::create_dir_all(dir).map_err(|e| format!("não consegui criar {}: {e}", dir.display()))?;
            }
            std::fs::rename(&from, &to).map_err(|e| format!("não consegui mover {} para {}: {e}", from.display(), to.display()))?;
        }
        self.forms[index] = moved;
        self.save()
    }

    /// Acrescenta um form novo e vazio (grava o `.json` em `forms/`). Devolve a entrada.
    pub fn add_form(&mut self, name: &str, caption: &str) -> Result<FormEntry, Error> {
        if !is_identifier(name) {
            return Err(format!("'{name}' não é um nome válido (letras, números e _)"));
        }
        if self.form(name).is_some() {
            return Err(format!("o projeto já tem um form '{name}'"));
        }
        let file = format!("forms/{}.json", file_stem_for(name));
        let path = self.file(&file);
        if path.exists() {
            return Err(format!("o arquivo {file} já existe"));
        }
        std::fs::create_dir_all(path.parent().expect("forms/ tem pai")).map_err(|e| e.to_string())?;
        std::fs::write(&path, empty_form(name, caption)).map_err(|e| format!("não consegui gravar {file}: {e}"))?;
        let entry = FormEntry { name: name.to_string(), file, auto_create: false, data_module: false, uses: Vec::new(), modified: None };
        self.forms.push(entry.clone());
        self.refresh_form(name);
        Ok(entry)
    }

    /// Acrescenta um DataModule novo e vazio (`forms/<nome>.json`, raiz `DataModule`),
    /// criado no início (o aplicativo cria os DataModules antes dos forms).
    pub fn add_data_module(&mut self, name: &str) -> Result<FormEntry, Error> {
        if !is_identifier(name) {
            return Err(format!("'{name}' não é um nome válido (letras, números e _)"));
        }
        if self.form(name).is_some() {
            return Err(format!("o projeto já tem um form ou DataModule '{name}'"));
        }
        let file = format!("forms/{}.json", file_stem_for(name));
        let path = self.file(&file);
        if path.exists() {
            return Err(format!("o arquivo {file} já existe"));
        }
        std::fs::create_dir_all(path.parent().expect("forms/ tem pai")).map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&serde_json::json!({ "aylis": 1, "root": { "class": "DataModule", "name": name } })).unwrap_or_default();
        std::fs::write(&path, json + "\n").map_err(|e| format!("não consegui gravar {file}: {e}"))?;
        let entry = FormEntry { name: name.to_string(), file, auto_create: true, data_module: true, uses: Vec::new(), modified: None };
        self.forms.push(entry.clone());
        self.refresh_form(name);
        Ok(entry)
    }

    /// A raiz do layout de uma entrada (o JSON lido do disco), se der para ler.
    fn root_of(&self, entry: &FormEntry) -> Option<serde_json::Value> {
        let text = std::fs::read_to_string(self.file(&entry.file)).ok()?;
        let mut value: serde_json::Value = serde_json::from_str(&text).ok()?;
        Some(value["root"].take())
    }

    /// Anota na entrada do form `name` o que o `.aylisproj` guarda dele (`data_module`, `uses` e a
    /// hora do arquivo), lendo o JSON dele. A IDE chama ao salvar o form.
    pub fn refresh_form(&mut self, name: &str) {
        let Some(i) = self.forms.iter().position(|f| f.name.eq_ignore_ascii_case(name)) else { return };
        self.refresh_entry(i);
    }

    fn refresh_entry(&mut self, i: usize) {
        let entry = &self.forms[i];
        let modified = modified_ms(&self.file(&entry.file));
        let root = self.root_of(entry);
        let entry = &mut self.forms[i];
        entry.data_module = root.as_ref().is_some_and(|r| r["class"] == "DataModule");
        entry.uses = root.as_ref().map(uses_in).unwrap_or_default();
        entry.modified = modified;
    }

    /// Lê de novo os forms cujo arquivo tem outra hora que a anotada (mudaram fora da IDE, ou o
    /// projeto é de antes desta anotação). Devolve se alguma entrada mudou.
    pub fn refresh_forms(&mut self) -> bool {
        let mut changed = false;
        for i in 0..self.forms.len() {
            if modified_ms(&self.file(&self.forms[i].file)) != self.forms[i].modified {
                let before = self.forms[i].clone();
                self.refresh_entry(i);
                changed |= self.forms[i] != before;
            }
        }
        changed
    }

    /// A entrada é um DataModule (a raiz do layout é `DataModule`)?
    pub fn is_data_module(&self, name: &str) -> bool {
        self.form(name).is_some_and(|e| e.data_module)
    }

    /// O [`variants_of`](Self::variants_of) de uma entrada já achada, com as pastas lidas uma vez.
    pub fn entry_variants(&self, entry: &FormEntry, files: &mut FormFiles) -> Vec<String> {
        VARIANTS.iter().filter(|v| files.exists(&self.file(&variant_file(&entry.file, v)))).map(|v| v.to_string()).collect()
    }

    /// As variantes do form que existem em disco (`android`, `desktop`...), na ordem de
    /// [`VARIANTS`].
    pub fn variants_of(&self, form: &str) -> Vec<String> {
        let Some(entry) = self.form(form) else { return Vec::new() };
        VARIANTS.iter().filter(|v| self.file(&variant_file(&entry.file, v)).is_file()).map(|v| v.to_string()).collect()
    }

    /// Cria a variante `variant` do form, copiando o layout padrão (o ponto de partida;
    /// depois cada uma tem o visual que quiser). Devolve o arquivo.
    pub fn add_variant(&self, form: &str, variant: &str) -> Result<String, Error> {
        let entry = self.form(form).ok_or_else(|| format!("o projeto não tem o form '{form}'"))?;
        if !VARIANTS.contains(&variant) {
            return Err(format!("variante desconhecida '{variant}' (use {})", VARIANTS.join(", ")));
        }
        let file = variant_file(&entry.file, variant);
        let path = self.file(&file);
        if path.exists() {
            return Err(format!("a variante {variant} de {form} já existe"));
        }
        std::fs::copy(self.file(&entry.file), &path).map_err(|e| format!("não consegui criar {file}: {e}"))?;
        Ok(file)
    }

    /// Os layouts do form: o padrão e as variantes (para juntar handlers e componentes).
    pub fn read_form_variants(&self, form: &str) -> Result<Vec<(Option<String>, String)>, Error> {
        let entry = self.form(form).ok_or_else(|| format!("o projeto não tem o form '{form}'"))?;
        let mut out = vec![(None, self.read_form(form)?)];
        for v in self.variants_of(form) {
            let path = self.file(&variant_file(&entry.file, &v));
            out.push((Some(v), std::fs::read_to_string(&path).map_err(|e| format!("não consegui ler {}: {e}", path.display()))?));
        }
        Ok(out)
    }

    /// Os DataModules que o form declara em `"uses"`.
    pub fn uses_of(&self, form: &str) -> Vec<String> {
        self.form(form).map(|e| e.uses.clone()).unwrap_or_default()
    }

    /// Os forms que usam o DataModule (o inverso de [`uses_of`](Self::uses_of)).
    pub fn used_by(&self, data_module: &str) -> Vec<String> {
        self.forms
            .iter()
            .filter(|f| self.uses_of(&f.name).iter().any(|u| u.eq_ignore_ascii_case(data_module)))
            .map(|f| f.name.clone())
            .collect()
    }

    /// Acrescenta um form que já existe em disco (copiado para `forms/` se estiver fora).
    pub fn add_existing_form(&mut self, layout: &Path) -> Result<FormEntry, Error> {
        let text = std::fs::read_to_string(layout).map_err(|e| format!("não consegui ler {}: {e}", layout.display()))?;
        let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        let name = value["root"]["name"].as_str().ok_or("o layout não tem o nome do form (root.name)")?.to_string();
        if self.form(&name).is_some() {
            return Err(format!("o projeto já tem um form '{name}'"));
        }
        let forms_dir = self.dir().join("forms");
        let inside = layout.canonicalize().ok().zip(forms_dir.canonicalize().ok()).is_some_and(|(l, f)| l.starts_with(f));
        let file = if inside {
            format!("forms/{}", layout.file_name().and_then(|n| n.to_str()).unwrap_or("form.json"))
        } else {
            let file = format!("forms/{}.json", file_stem_for(&name));
            std::fs::create_dir_all(&forms_dir).map_err(|e| e.to_string())?;
            std::fs::write(self.file(&file), text).map_err(|e| e.to_string())?;
            file
        };
        let entry = FormEntry { name, file, auto_create: false, data_module: false, uses: Vec::new(), modified: None };
        self.forms.push(entry.clone());
        self.refresh_form(&entry.name.clone());
        Ok(entry)
    }

    /// Tira o form do projeto (o arquivo continua no disco). O principal não pode sair.
    pub fn remove_form(&mut self, name: &str) -> Result<FormEntry, Error> {
        if self.main_form.eq_ignore_ascii_case(name) {
            return Err("o form principal não pode ser removido (escolha outro principal antes)".into());
        }
        let index = self.forms.iter().position(|f| f.name.eq_ignore_ascii_case(name)).ok_or_else(|| format!("o projeto não tem o form '{name}'"))?;
        Ok(self.forms.remove(index))
    }

    /// Troca o nome de um form no projeto (quando o form é renomeado no designer).
    pub fn rename_form(&mut self, old: &str, new: &str) {
        if let Some(entry) = self.forms.iter_mut().find(|f| f.name.eq_ignore_ascii_case(old)) {
            entry.name = new.to_string();
        }
        if self.main_form.eq_ignore_ascii_case(old) {
            self.main_form = new.to_string();
        }
    }
}

/// `frmClientes` → `clientes` (nome do arquivo do form).
pub fn file_stem_for(form_name: &str) -> String {
    let lower = form_name.to_lowercase();
    let stem = ["frm", "form", "dm"].iter().find_map(|p| lower.strip_prefix(p).filter(|r| !r.is_empty())).unwrap_or(&lower);
    stem.trim_start_matches('_').to_string()
}

/// O que a pasta `.aylis` de um projeto diz ao git: tudo nela é desta máquina (a junção do
/// SDK, as horas dos forms, os marcadores, a casca e as compilações), e o próprio
/// `.gitignore` também.
pub const LOCAL_DIR_IGNORE: &str = "# Gravado pelo Aylis: tudo nesta pasta é desta máquina e fica fora do git (sdk,\n# form-stamps.json, bookmarks.json, shell/, build/).\n*\n";

/// Deixa o `.gitignore` da pasta `.aylis` (`dir`) ignorando tudo, de uma vez: cada recurso
/// acrescentava a regra dele quando aparecia (`sdk`, depois `form-stamps.json`...), e o arquivo,
/// rastreado no projeto, mudava a cada versão (I06 do Tabernáculo). Já ignorando tudo: não mexe.
pub fn local_dir_ignored(dir: &Path) {
    let ignore = dir.join(".gitignore");
    let rules = std::fs::read_to_string(&ignore).unwrap_or_default();
    if rules.lines().any(|l| l.trim() == "*") {
        return;
    }
    let _ = std::fs::create_dir_all(dir);
    // As regras de antes ficam (o `*` cobre todas).
    let text = if rules.trim().is_empty() { LOCAL_DIR_IGNORE.to_string() } else { format!("{}\n*\n", rules.trim_end()) };
    let _ = std::fs::write(&ignore, text);
}

fn empty_form(name: &str, caption: &str) -> String {
    let doc = serde_json::json!({
        "aylis": 1,
        "root": { "class": "Form", "name": name, "caption": caption, "width": 800, "height": 600, "padding": 8 }
    });
    serde_json::to_string_pretty(&doc).expect("JSON válido") + "\n"
}

/// Um caminho relativo que não sai da pasta (nada de absoluto, `C:`, nem `..`).
pub fn inside_project(relative: &str) -> bool {
    !relative.is_empty() && Path::new(relative).components().all(|c| matches!(c, std::path::Component::Normal(_) | std::path::Component::CurDir))
}

/// Identificador: letra ou `_`, depois letras, dígitos ou `_`.
pub fn is_identifier(name: &str) -> bool {
    aylis_core::is_identifier(name)
}

/// Cria um projeto novo: a pasta `<local>/<nome>`, as subpastas, o `.aylisproj`, o form
/// principal (`forms/principal.json`, `frmPrincipal`) e o código inicial na linguagem
/// escolhida (na pasta dela, como `rust/`). A pasta não pode existir com arquivos dentro.
/// O valor da chave `sdk` que usa o Aylis publicado no crates.io.
pub const SDK_CRATES_IO: &str = "crates.io";

pub fn create(location: &Path, name: &str, language: Language) -> Result<Project, Error> {
    let dir = location.join(name);
    if dir.read_dir().is_ok_and(|mut entries| entries.next().is_some()) {
        return Err(format!("a pasta {} já existe e não está vazia", dir.display()));
    }
    create_at(&dir, name, language)
}

/// O [`create`] direto na pasta `dir`, que pode já existir com outras coisas (o app dentro de
/// um repositório: `RTSPlayer/client`, ao lado do `core/`). Recusa se já há ali um projeto,
/// forms ou a pasta do código na linguagem. Sem isso, criava-se noutra pasta e movia-se depois,
/// e o projeto movido abria em modo restrito (relatório do RTSPlayer, 10/11).
pub fn create_at(dir: &Path, name: &str, language: Language) -> Result<Project, Error> {
    let valid = !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == ' ');
    if !valid || name.trim() != name {
        return Err(format!("'{name}' não serve como nome de projeto (use letras, números, espaço, - e _)"));
    }
    let has_project = dir.read_dir().is_ok_and(|entries| entries.flatten().any(|e| e.path().extension().is_some_and(|x| x == EXTENSION)));
    let has_forms = dir.join("forms").read_dir().is_ok_and(|mut entries| entries.next().is_some());
    if has_project || has_forms || dir.join(language.folder()).exists() {
        return Err(format!("{} já tem um projeto do Aylis (o .aylisproj, forms/ ou {}/)", dir.display(), language.folder()));
    }
    for sub in ["forms", "assets/images", "assets/audio", "assets/video", "assets/fonts", "themes", "locales"] {
        std::fs::create_dir_all(dir.join(sub)).map_err(|e| format!("não consegui criar {}: {e}", dir.join(sub).display()))?;
    }
    let mut project = Project {
        format: FORMAT_VERSION,
        name: name.to_string(),
        language,
        main_form: "frmPrincipal".into(),
        forms: Vec::new(),
        theme: String::new(),
        colors: Default::default(),
        scroll: None,
        zoom: None,
        time_zone: String::new(),
        features: Vec::new(),
        limits: None,
        image_cache: None,
        output: OutputSettings::default(),
        fonts: Default::default(),
        theme_mix: Default::default(),
        font: String::new(),
        locale: "pt-BR".into(),
        publish: PublishOptions::default(),
        // Projetos novos: Direct2D no Windows, Skia nas outras.
        render: Some(RenderOptions::default()),
        text: Some(TextOptions::default()),
        gpu: String::new(),
        present: String::new(),
        // Projetos novos: o Aylis do crates.io, que compila em qualquer máquina (G11).
        sdk: SDK_CRATES_IO.into(),
        dev_address: String::new(),
        platforms: Vec::new(),
        device: String::new(),
        path: dir.join(format!("{name}.{EXTENSION}")),
    };
    project.add_form("frmPrincipal", name)?;
    project.forms[0].auto_create = true;
    // Uma fonte embutida como padrão: a do sistema muda de uma plataforma para outra (e com
    // ela as medidas e as quebras de linha); a embutida é a mesma em todas.
    if let Some(entry) = assets::font_catalog().into_iter().find(|f| f.family == DEFAULT_FONT) {
        assets::add_font(&project, &entry)?;
        project.font = DEFAULT_FONT.into();
    }
    project.save()?;
    codegen::generate(&project)?;
    Ok(project)
}

/// Um modelo do "Novo projeto": em branco, já configurado para onde o app vai rodar
/// (decidido pelo autor em 08/11, no lugar do "Em branco" e do "Landing page", que abria um
/// projeto cheio de coisas).
pub struct Template {
    /// O nome que aparece.
    pub name: &'static str,
    /// O que o assistente diz dele.
    pub description: &'static str,
    /// As plataformas que o assistente marca (as chaves de [`new_project::TARGETS`]).
    pub platforms: &'static [&'static str],
    /// O tamanho do form principal.
    pub size: (f32, f32),
    /// O aparelho do designer e do Executar (a moldura); vazio: nenhum.
    pub device: &'static str,
}

/// Os modelos do "Novo projeto".
pub const TEMPLATES: &[Template] = &[
    Template {
        name: "Desktop",
        description: "Uma janela de computador (Windows, Linux e macOS), de 1024 × 640.",
        platforms: &["windows", "linux", "macos"],
        size: (1024.0, 640.0),
        device: "",
    },
    Template {
        name: "Smartphone",
        description: "Uma tela de celular (Android e iOS), de 412 × 915, com a moldura do aparelho no designer e ao executar.",
        platforms: &["android", "ios"],
        size: (412.0, 915.0),
        device: "Pixel 8",
    },
    Template {
        name: "Web",
        description: "Uma página no navegador, de 1280 × 800 (a página ocupa a janela do navegador).",
        platforms: &["web"],
        size: (1280.0, 800.0),
        device: "",
    },
];

/// O modelo pelo nome (sem diferença de maiúsculas).
pub fn template(name: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|t| t.name.eq_ignore_ascii_case(name.trim()))
}

/// Configura o projeto recém-criado pelo modelo `template` (o nome de [`TEMPLATES`]): as
/// plataformas, o tamanho do form principal e o aparelho.
pub fn apply_template(project: &mut Project, template: &str) -> Result<(), Error> {
    let t = self::template(template).ok_or_else(|| format!("modelo desconhecido: {template}"))?;
    project.platforms = t.platforms.iter().map(|p| p.to_string()).collect();
    project.device = t.device.to_string();
    let entry = project.form(&project.main_form).ok_or_else(|| format!("o projeto não tem o form '{}'", project.main_form))?;
    let path = project.file(&entry.file);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("não consegui ler {}: {e}", path.display()))?;
    let mut json: serde_json::Value = serde_json::from_str(text.trim_start_matches('\u{feff}')).map_err(|e| format!("{}: {e}", path.display()))?;
    json["root"]["width"] = serde_json::json!(t.size.0);
    json["root"]["height"] = serde_json::json!(t.size.1);
    let text = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?;
    std::fs::write(&path, text + "\n").map_err(|e| format!("não consegui gravar {}: {e}", path.display()))?;
    project.save()?;
    Ok(())
}

/// Põe no projeto recém-criado o conteúdo de um exemplo do Aylis (`examples/<folder>`): os
/// forms, os assets, os temas, as traduções, o tema e, em Rust, os handlers dos forms; nas
/// outras linguagens a geração cria os handlers vazios. Para os testes e para quem quer
/// partir de um exemplo.
pub fn apply_example(project: &mut Project, folder: &str) -> Result<(), Error> {
    let source = Path::new(&codegen::aylis_root()).join("examples").join(folder);
    let model = Project::open(source.join(format!("{folder}.{EXTENSION}")))?;
    fn copy_tree(from: &Path, to: &Path) -> Result<(), Error> {
        for entry in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))?.flatten() {
            let (src, dst) = (entry.path(), to.join(entry.file_name()));
            if src.is_dir() {
                std::fs::create_dir_all(&dst).map_err(|e| format!("{}: {e}", dst.display()))?;
                copy_tree(&src, &dst)?;
            } else {
                std::fs::copy(&src, &dst).map_err(|e| format!("{}: {e}", dst.display()))?;
            }
        }
        Ok(())
    }
    for sub in ["forms", "assets", "themes", "locales"] {
        if source.join(sub).is_dir() {
            copy_tree(&source.join(sub), &project.dir().join(sub))?;
        }
    }
    project.main_form = model.main_form.clone();
    project.forms = model.forms.clone();
    project.theme = model.theme.clone();
    if project.language == Language::Rust {
        let (from, to) = (model.code_dir_of(Language::Rust).join("src/forms"), project.code_dir_of(Language::Rust).join("src/forms"));
        for form in &model.forms {
            let stem = Path::new(&form.file).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
            let file = from.join(format!("{stem}.rs"));
            if file.is_file() {
                std::fs::copy(&file, to.join(format!("{stem}.rs"))).map_err(|e| format!("{}: {e}", file.display()))?;
            }
        }
    }
    project.save()?;
    codegen::generate(project)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Serviços para o Ui

/// Lê os arquivos do aplicativo de uma pasta (a do projeto).
pub struct DirResources(pub PathBuf);

impl ResourceService for DirResources {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        // Só dentro da pasta: nada de "../" para sair dela.
        if Path::new(path).components().any(|c| !matches!(c, std::path::Component::Normal(_) | std::path::Component::CurDir)) {
            return None;
        }
        std::fs::read(self.0.join(path)).ok()
    }
}

/// Os arquivos do projeto embutidos no executável (o pacote feito pelo `aylis-build`).
#[derive(Clone, Default)]
pub struct Bundle {
    files: Arc<HashMap<String, Vec<u8>>>,
}

impl Bundle {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let files = aylis_build::unpack(bytes)?.into_iter().map(|(name, data)| (name, data.to_vec())).collect();
        Ok(Self { files: Arc::new(files) })
    }

    pub fn get(&self, path: &str) -> Option<&[u8]> {
        self.files.get(path.trim_start_matches("./")).map(Vec::as_slice)
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.files.keys().map(String::as_str)
    }

    /// O projeto de dentro do pacote (o `path` fica só com o nome do `.aylisproj`).
    pub fn project(&self) -> Result<Project, Error> {
        let (name, text) = self
            .files
            .iter()
            .find(|(n, _)| n.ends_with(&format!(".{EXTENSION}")) && !n.contains('/'))
            .ok_or("o pacote não tem o .aylisproj")?;
        let text = text.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(text);
        let mut project: Project = serde_json::from_slice(text).map_err(|e| format!("{name}: {e}"))?;
        project.path = PathBuf::from(name);
        Ok(project)
    }
}

impl ResourceService for Bundle {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        self.get(path).map(<[u8]>::to_vec)
    }
}

/// De onde vêm os arquivos do projeto: a pasta (desenvolvimento) ou o pacote embutido.
#[derive(Clone)]
pub enum Files {
    Dir(PathBuf),
    Bundle(Bundle),
}

impl Files {
    pub fn read(&self, path: &str) -> Option<Vec<u8>> {
        match self {
            Files::Dir(dir) => DirResources(dir.clone()).read(path),
            Files::Bundle(b) => b.read(path),
        }
    }

    /// Os arquivos (caminhos relativos, com `/`) que começam com `prefix`, em ordem.
    pub fn list(&self, prefix: &str) -> Vec<String> {
        let mut out: Vec<String> = match self {
            Files::Bundle(b) => b.names().filter(|n| n.starts_with(prefix)).map(str::to_string).collect(),
            Files::Dir(dir) => {
                fn walk(base: &Path, dir: &Path, out: &mut Vec<String>) {
                    let Ok(entries) = std::fs::read_dir(dir) else { return };
                    for e in entries.flatten() {
                        let p = e.path();
                        if p.is_dir() {
                            walk(base, &p, out);
                        } else if let Ok(rel) = p.strip_prefix(base) {
                            out.push(rel.to_string_lossy().replace('\\', "/"));
                        }
                    }
                }
                let mut out = Vec::new();
                walk(dir, &dir.join(prefix.trim_end_matches('/')), &mut out);
                out
            }
        };
        out.sort();
        out
    }

    pub fn read_text(&self, path: &str) -> Result<String, Error> {
        let bytes = self.read(path).ok_or_else(|| format!("arquivo do projeto não encontrado: {path}"))?;
        String::from_utf8(bytes).map_err(|_| format!("{path} não é texto UTF-8"))
    }
}

impl ResourceService for Files {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        Files::read(self, path)
    }
}

/// As variantes de um form por plataforma: `forms/principal.android.json` vale no
/// Android no lugar de `forms/principal.json`. As de sistema vencem as de grupo
/// (`android` antes de `mobile`); sem variante, vale o arquivo padrão.
pub const VARIANTS: [&str; 8] = ["windows", "macos", "linux", "android", "ios", "desktop", "mobile", "web"];

/// A marca de um arquivo: a hora da última alteração (em nanossegundos) e o tamanho. Para
/// saber se um arquivo mudou desde a última vez, compare a marca guardada com a de agora
/// (diferente = mudou), e não a hora dele com a de um marcador: gravado no mesmo
/// milissegundo do marcador (ou num disco de relógio grosso, de rede ou FAT), o arquivo
/// parecia não ter mudado.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStamp {
    pub nanos: u64,
    pub len: u64,
}

impl FileStamp {
    pub fn of(meta: &std::fs::Metadata) -> Option<FileStamp> {
        let nanos = meta.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok()?.as_nanos() as u64;
        Some(FileStamp { nanos, len: meta.len() })
    }

    pub fn read(path: &Path) -> Option<FileStamp> {
        FileStamp::of(&std::fs::metadata(path).ok()?)
    }

    pub fn millis(self) -> u64 {
        self.nanos / 1_000_000
    }
}

/// Os arquivos das pastas de forms, cada pasta lida uma vez: quem percorre todos os forms
/// procurando as variantes perguntava ao disco por cada uma (8 por form: no ERP de 1009 forms,
/// 8 mil consultas, lentas numa pasta do OneDrive).
#[derive(Default)]
pub struct FormFiles {
    /// Pasta → nome → a marca do arquivo (a hora e o tamanho, que a listagem da pasta já traz).
    dirs: std::collections::HashMap<PathBuf, std::collections::HashMap<String, Option<FileStamp>>>,
}

impl FormFiles {
    /// O nome como o sistema compara (sem diferenciar maiúsculas no Windows e no macOS).
    fn key(name: &std::ffi::OsStr) -> String {
        let name = name.to_string_lossy();
        if cfg!(any(windows, target_os = "macos")) { name.to_lowercase() } else { name.into_owned() }
    }

    fn entry(&mut self, path: &Path) -> Option<Option<FileStamp>> {
        let (Some(dir), Some(name)) = (path.parent(), path.file_name()) else { return None };
        let names = self.dirs.entry(dir.to_path_buf()).or_insert_with(|| {
            std::fs::read_dir(dir).map(|r| r.flatten().map(|e| (Self::key(&e.file_name()), e.metadata().ok().and_then(|m| FileStamp::of(&m)))).collect()).unwrap_or_default()
        });
        names.get(&Self::key(name)).copied()
    }

    pub fn exists(&mut self, path: &Path) -> bool {
        self.entry(path).is_some()
    }

    /// A hora da última alteração do arquivo (milissegundos desde 1970), pela listagem da pasta.
    pub fn modified(&mut self, path: &Path) -> Option<u64> {
        self.entry(path).flatten().map(FileStamp::millis)
    }

    /// A marca do arquivo (a hora exata e o tamanho), pela listagem da pasta.
    pub fn stamp(&mut self, path: &Path) -> Option<FileStamp> {
        self.entry(path).flatten()
    }

    /// Esquece a pasta (ela mudou: a próxima pergunta lista de novo).
    pub fn forget(&mut self, dir: &Path) {
        self.dirs.remove(dir);
    }
}

impl Project {
    /// O [`read_form_variants`](Self::read_form_variants) de uma entrada já achada, com as
    /// pastas lidas uma vez em `files` (para percorrer todos os forms).
    pub fn read_entry_variants(&self, entry: &FormEntry, files: &mut FormFiles) -> Result<Vec<(Option<String>, String)>, Error> {
        let read = |path: &Path| std::fs::read_to_string(path).map_err(|e| format!("não consegui ler {}: {e}", path.display()));
        let mut out = vec![(None, read(&self.file(&entry.file))?)];
        for v in VARIANTS {
            let path = self.file(&variant_file(&entry.file, v));
            if files.exists(&path) {
                out.push((Some(v.to_string()), read(&path)?));
            }
        }
        Ok(out)
    }
}

/// O arquivo da variante: `forms/principal.json` + `android` → `forms/principal.android.json`.
pub fn variant_file(file: &str, variant: &str) -> String {
    match file.strip_suffix(".json") {
        Some(stem) => format!("{stem}.{variant}.json"),
        None => format!("{file}.{variant}"),
    }
}

/// As variantes que valem no sistema em que o programa roda, da mais específica à mais
/// geral.
pub fn current_variants() -> &'static [&'static str] {
    if cfg!(target_arch = "wasm32") {
        &["web"]
    } else if cfg!(target_os = "android") {
        &["android", "mobile"]
    } else if cfg!(target_os = "ios") {
        &["ios", "mobile"]
    } else if cfg!(windows) {
        &["windows", "desktop"]
    } else if cfg!(target_os = "macos") {
        &["macos", "desktop"]
    } else {
        &["linux", "desktop"]
    }
}

/// O arquivo do form que vale neste sistema (a primeira variante que existe, senão o
/// padrão), entre os arquivos de `exists`.
pub fn resolve_variant(file: &str, exists: impl Fn(&str) -> bool) -> String {
    current_variants().iter().map(|v| variant_file(file, v)).find(|f| exists(f)).unwrap_or_else(|| file.to_string())
}

/// Os nomes em `"uses"` de uma raiz de layout.
pub fn uses_in(root: &serde_json::Value) -> Vec<String> {
    root["uses"].as_array().map(|list| list.iter().filter_map(|v| v.as_str()).map(str::to_string).collect()).unwrap_or_default()
}

/// Cria os forms do projeto pelo nome (carregando o `.json` de cada um). O terceiro campo
/// guarda o JSON já lido de cada form: um frame repetido (um cartão por item,
/// `FraX::create_in`) não relê nem reinterpreta o texto a cada instância (G21 do
/// Tabernáculo: da pasta, o `create_in` relia e interpretava o arquivo a cada cartão).
pub struct ProjectForms(pub Project, pub Files, pub HashMap<String, CachedForm>);

/// O JSON já lido de um form (pela entrada do projeto: `forms/x.json`). Do pacote embutido o
/// arquivo não muda. Da pasta (a feature `dev`, a IDE) vale enquanto a marca do arquivo
/// (`FileStamp`) for a mesma, e a marca é conferida uma vez por instante do relógio do `Ui`
/// (`Ui::time`, que a plataforma anda a cada evento): os 200 cartões de um handler perguntam
/// ao disco uma vez, e não 600 (cada pergunta custa dezenas de µs no Windows).
pub struct CachedForm {
    /// O arquivo lido (a variante deste sistema, se havia uma).
    file: String,
    stamp: Option<FileStamp>,
    /// O `Ui::time` da última conferência (sem relógio, 0: confere sempre).
    checked: f64,
    value: std::rc::Rc<serde_json::Value>,
}

impl ProjectForms {
    /// O arquivo existe? Com a marca dele, se vem da pasta (o pacote não tem marca).
    fn stamp(&self, file: &str) -> Option<Option<FileStamp>> {
        match &self.1 {
            Files::Bundle(b) => b.get(file).map(|_| None),
            // Só dentro da pasta, como o `DirResources`.
            Files::Dir(dir) => match Path::new(file).components().all(|c| matches!(c, std::path::Component::Normal(_) | std::path::Component::CurDir)) {
                true => std::fs::metadata(dir.join(file)).ok().filter(|m| m.is_file()).map(|m| FileStamp::of(&m)),
                false => None,
            },
        }
    }

    /// O JSON do form da entrada `entry_file` (na variante deste sistema, se houver), do que
    /// já foi lido quando o arquivo não mudou.
    fn form_value(&mut self, entry_file: &str, now: f64) -> Result<std::rc::Rc<serde_json::Value>, String> {
        let bundle = matches!(self.1, Files::Bundle(_));
        if let Some(c) = self.2.get(entry_file) {
            if bundle || (now > 0.0 && c.checked == now) {
                return Ok(c.value.clone());
            }
        }
        // A variante deste sistema (`principal.android.json`), se houver (o `resolve_variant`,
        // guardando a marca de quem existe): a pergunta é pela marca, sem ler o conteúdo.
        let variant = current_variants().iter().find_map(|v| {
            let f = variant_file(entry_file, v);
            self.stamp(&f).map(|s| (f, s))
        });
        let (file, stamp) = variant.unwrap_or_else(|| (entry_file.to_string(), self.stamp(entry_file).flatten()));
        if let Some(c) = self.2.get_mut(entry_file) {
            // Sem marca (o sistema não deu a hora): lido de novo, por garantia.
            if c.file == file && stamp.is_some() && c.stamp == stamp {
                c.checked = now;
                return Ok(c.value.clone());
            }
        }
        let json = self.1.read_text(&file)?;
        let value = std::rc::Rc::new(serde_json::from_str::<serde_json::Value>(&json).map_err(|e| format!("{file}: {e}"))?);
        self.2.insert(entry_file.to_string(), CachedForm { file, stamp, checked: now, value: value.clone() });
        Ok(value)
    }

    /// Carrega o form `name` e os frames dele (cada `Frame` ganha o form que cita).
    fn load_form(&mut self, ui: &mut Ui, name: &str, loading: &mut Vec<String>) -> Result<ComponentId, String> {
        let entry = self.0.form(name).ok_or_else(|| format!("o projeto não tem o form '{name}'"))?;
        let entry_file = entry.file.clone();
        let value = self.form_value(&entry_file, ui.time())?;
        // Os DataModules do `uses` antes (as referências `dmDados.x` precisam deles).
        for module in uses_in(&value["root"]) {
            if !module.eq_ignore_ascii_case(name) {
                ui.create_form(&module).map_err(|e| e.to_string())?;
            }
        }
        let form = aylis_json::load_value(ui, &value, None).map_err(|e| e.to_string())?;
        if !ui.name(form).is_some_and(|n| n.eq_ignore_ascii_case(name)) {
            ui.free(form);
            return Err(format!("o layout de '{name}' tem outro nome na raiz"));
        }
        if ui.class_name(form) == Some("DataModule") {
            ui.fire(form, "on_create", &aylis_core::EventArgs::None);
        }
        loading.push(name.to_string());
        let embedded = self.embed_frames(ui, form, loading);
        loading.pop();
        // A instância do form (a struct gerada): o form que não bate com o código não abre.
        if let Err(e) = embedded.and_then(|_| ui.bind_form(form)) {
            ui.free(form);
            return Err(e);
        }
        Ok(form)
    }

    /// Põe em cada `Frame` de `root` o form dele, ocupando a área toda e, sem fundo próprio,
    /// transparente. Cada form entra uma vez só (o nome dele é o caminho para os
    /// componentes); `loading` são os forms sendo carregados, um dentro do outro.
    fn embed_frames(&mut self, ui: &mut Ui, root: ComponentId, loading: &mut Vec<String>) -> Result<(), String> {
        let mut frames = Vec::new();
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            if ui.class_name(id) == Some("Frame") {
                if let Ok(aylis_core::PropValue::String(form)) = ui.get_prop(id, "form") {
                    if !form.is_empty() && ui.children(id).is_empty() {
                        frames.push((id, form));
                    }
                }
                continue;
            }
            stack.extend(ui.children(id).iter().copied());
        }
        for (frame, name) in frames {
            // Um pacote (`pkg:Termometro`): cada uso é um form à parte, com os valores das
            // propriedades expostas.
            if let Some(package) = packages::package_of(&name) {
                let manifest = packages::manifest(&self.1, package).ok_or_else(|| format!("o pacote '{package}' não está no projeto (packages/{package})"))?;
                let values = match ui.get_prop(frame, "values") {
                    Ok(aylis_core::PropValue::String(v)) => v,
                    _ => String::new(),
                };
                // Tipo B: a classe do crate, registrada pelo `register` dele (o código gerado do
                // projeto Rust chama). Sem ela (o app de outra linguagem), o frame fica vazio.
                if manifest.is_code() {
                    if ui.registry().get(&manifest.component).is_none() {
                        aylis_core::stderr_line!("aylis: pacote {package}: a classe {} não está registrada (o pacote de código vale no app em Rust)", manifest.component);
                        continue;
                    }
                    let inner = ui.create(&manifest.component, None).map_err(|e| format!("pacote {package}: {e}"))?;
                    let name = packages::inner_name(ui, frame, package);
                    let _ = ui.set_name(inner, &name);
                    for w in packages::apply_values(ui, inner, &manifest, &values).into_iter().chain(packages::apply_events(ui, inner, frame, &manifest, &values)) {
                        aylis_core::stderr_line!("aylis: pacote {package}: {w}");
                    }
                    ui.set_parent(inner, Some(frame)).map_err(|e| e.to_string())?;
                    let _ = ui.set_prop(inner, "align", "client");
                    continue;
                }
                let inner = packages::inner_name(ui, frame, package);
                let layout = packages::form_layout(&self.1, package, &inner)?;
                let form = aylis_json::load_value(ui, &layout, None).map_err(|e| format!("pacote {package}: {e}"))?;
                for w in packages::apply_values(ui, form, &manifest, &values).into_iter().chain(packages::apply_events(ui, form, frame, &manifest, &values)) {
                    aylis_core::stderr_line!("aylis: pacote {package}: {w}");
                }
                ui.set_parent(form, Some(frame)).map_err(|e| e.to_string())?;
                let _ = ui.set_prop(form, "align", "client");
                if matches!(ui.get_prop(form, "fill"), Ok(aylis_core::PropValue::Null)) {
                    let _ = ui.set_prop(form, "fill", aylis_core::Color::TRANSPARENT);
                }
                if let Some(cb) = ui.control_mut(form) {
                    cb.set_visible(true);
                }
                continue;
            }
            if loading.iter().any(|l| l.eq_ignore_ascii_case(&name)) {
                return Err(format!("o frame '{name}' entra nele mesmo ({} → {name})", loading.join(" → ")));
            }
            if ui.find_path(&name).is_some() {
                return Err(format!("o form '{name}' já está em outro frame (cada form entra uma vez só)"));
            }
            let form = self.load_form(ui, &name, loading).map_err(|e| format!("frame {name}: {e}"))?;
            ui.set_parent(form, Some(frame)).map_err(|e| e.to_string())?;
            let _ = ui.set_prop(form, "align", "client");
            if matches!(ui.get_prop(form, "fill"), Ok(aylis_core::PropValue::Null)) {
                let _ = ui.set_prop(form, "fill", aylis_core::Color::TRANSPARENT);
            }
            if let Some(cb) = ui.control_mut(form) {
                cb.set_visible(true);
            }
        }
        Ok(())
    }
}

impl FormFactory for ProjectForms {
    fn create_form(&mut self, ui: &mut Ui, name: &str) -> Result<ComponentId, String> {
        self.load_form(ui, name, &mut Vec::new())
    }

    fn form_names(&self) -> Vec<String> {
        self.0.forms.iter().map(|f| f.name.clone()).collect()
    }
}

/// Prepara o `Ui` para o projeto: tema, idioma, arquivos do projeto e fábrica de forms;
/// cria os forms marcados como "criar no início" e devolve o form principal (criado).
///
/// As classes de componentes precisam estar registradas antes (`aylis_controls::register`...).
pub fn install(ui: &mut Ui, project: &Project) -> Result<ComponentId, Error> {
    install_with(ui, project, Files::Dir(project.dir()))
}

/// Como [`install`], com os arquivos vindos de `files` (a pasta ou o pacote embutido).
/// O tema do projeto: o do aylis, um de plataforma (pelo nome) ou um arquivo, já com a
/// mistura por classe (`theme_mix`).
pub fn resolve_theme(project: &Project, files: &Files) -> Result<Theme, Error> {
    let mut theme = match project.theme.as_str() {
        "" | "light" => Theme::light(),
        "dark" => Theme::dark(),
        name if aylis_core::themes::builtin(name).is_some() => aylis_core::themes::builtin(name).cloned().unwrap_or_default(),
        file => {
            let json = files.read_text(file).map_err(|e| format!("tema: {e}"))?;
            aylis_json::load_theme(&json).map_err(|e| format!("tema {file}: {e}"))?
        }
    };
    for (class, name) in &project.theme_mix {
        theme.mix(class, name);
    }
    theme.apply_palette(&project.palette());
    if !project.font.is_empty() {
        for font in theme.fonts.values_mut() {
            if font.family != "monospace" && !font.family.to_lowercase().contains("mono") {
                font.family = project.font.clone();
            }
        }
    }
    Ok(theme)
}

/// As fontes nomeadas do projeto (`fonts` do `.aylisproj`) no `Ui` (o app e o designer da IDE).
pub fn install_font_names(ui: &mut Ui, project: &Project) {
    ui.set_font_names(project.fonts.iter().map(|(n, f)| (n.clone(), f.clone())).collect());
}

/// Registra as fontes de `assets/fonts/` (TTF, OTF, TTC) e os pacotes de ícones de
/// `assets/icons/` (o nome do arquivo é o prefixo: `material.txt` → `material:home`).
pub fn install_assets(ui: &mut Ui, files: &Files) {
    for name in files.list("assets/fonts/") {
        let lower = name.to_lowercase();
        if lower.ends_with(".ttf") || lower.ends_with(".otf") || lower.ends_with(".ttc") {
            if let Some(data) = files.read(&name) {
                aylis_core::fonts::register(data);
            }
        }
    }
    for name in files.list("assets/icons/") {
        if let Some(stem) = name.strip_prefix("assets/icons/").and_then(|n| n.strip_suffix(".txt")) {
            if let Ok(text) = files.read_text(&name) {
                if let Err(e) = ui.register_icon_pack(stem, &text) {
                    aylis_core::stderr_line!("aylis: ícones {name}: {e}");
                }
            }
        }
    }
}

pub fn install_with(ui: &mut Ui, project: &Project, files: Files) -> Result<ComponentId, Error> {
    install_assets(ui, &files);
    let theme = resolve_theme(project, &files)?;
    ui.set_palette(project.palette());
    if let Some(s) = project.scroll {
        ui.set_scroll_options(aylis_core::ScrollOptions { wheel_step: s.wheel_step, smooth: s.smooth, smooth_ms: s.smooth_ms });
    }
    install_font_names(ui, project);
    if let Some(l) = project.limits {
        for warning in l.warnings() {
            aylis_core::stderr_line!("aylis: {warning}");
        }
        ui.set_limits(l.tree_depth, l.event_nesting);
    }
    let image_cache = project.image_cache.unwrap_or_default();
    aylis_core::image_cache::set_limits(image_cache.limits());
    aylis_core::image_cache::set_auto_layers(image_cache.auto_layers);
    if let Some(z) = project.zoom {
        ui.set_zoom(z.value);
        ui.set_zoom_keys(z.keys);
    }
    ui.set_theme(theme);
    // O idioma: `AYLIS_LANG` (para testar) ou o do projeto.
    let language = std::env::var("AYLIS_LANG").unwrap_or_else(|_| project.locale.clone());
    ui.services_mut().locale = Some(Box::new(i18n::locale_info(&language)));
    aylis_core::messages::set_language(&language);
    if let Some(text) = files.read(&i18n::locale_file(&language)) {
        ui.set_translations(i18n::parse(&String::from_utf8_lossy(&text)));
    }
    ui.services_mut().resources = Some(Box::new(files.clone()));
    ui.services_mut().forms = Some(Box::new(ProjectForms(project.clone(), files.clone(), HashMap::new())));
    // Os DataModules primeiro (os forms usam os componentes deles).
    let (modules, forms): (Vec<&FormEntry>, Vec<&FormEntry>) = project.forms.iter().filter(|f| f.auto_create).partition(|f| {
        files.read_text(&f.file).ok().and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok()).is_some_and(|v| v["root"]["class"] == "DataModule")
    });
    for entry in modules.into_iter().chain(forms) {
        ui.create_form(&entry.name).map_err(|e| e.to_string())?;
    }
    let main = ui.create_form(&project.main_form).map_err(|e| e.to_string())?;
    ui.set_main_form(main);
    Ok(main)
}

// ---------------------------------------------------------------------------
// Hot reload

/// Observa os arquivos do projeto (forms e tema) e, quando um muda no disco, aplica a
/// mudança no aplicativo em execução: o form aberto é atualizado no lugar (dados, foco e o
/// que foi digitado ficam; ver [`aylis_json::reload`]) e o tema é trocado. Para o
/// desenvolvimento: salvou na IDE, aparece no app.
///
/// Confere a cada 400 ms numa thread própria (sem custo perceptível); as mudanças são
/// aplicadas na thread da interface. Mensagens vão para o stderr (a IDE mostra em
/// Mensagens).
pub fn hot_reload(ui: &mut Ui, project: &Project) {
    let mut files: Vec<(Option<String>, PathBuf)> = project
        .forms
        .iter()
        .map(|f| (Some(f.name.clone()), project.file(&resolve_variant(&f.file, |v| project.file(v).is_file()))))
        .collect();
    let theme_file = project.theme.ends_with(".json");
    if theme_file {
        files.push((None, project.file(&project.theme)));
    }
    // A tradução do idioma atual também (o arquivo pode ainda não existir).
    let language = std::env::var("AYLIS_LANG").unwrap_or_else(|_| project.locale.clone());
    files.push((None, project.file(&i18n::locale_file(&language))));
    let poster = ui.poster();
    let mix = project.theme_mix.clone();
    std::thread::Builder::new()
        .name("aylis-hot-reload".into())
        .spawn(move || {
            let read = |p: &Path| std::fs::read_to_string(p).ok();
            let mut last: Vec<Option<String>> = files.iter().map(|(_, p)| read(p)).collect();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(400));
                for (i, (form, path)) in files.iter().enumerate() {
                    let Some(now) = read(path) else { continue };
                    if last[i].as_deref() == Some(now.as_str()) {
                        continue;
                    }
                    let previous = last[i].replace(now.clone());
                    let form = form.clone();
                    let file = path.display().to_string();
                    let is_locale = path.parent().and_then(Path::file_name).is_some_and(|d| d == "locales");
                    let mix = mix.clone();
                    poster.post(move |ui| {
                        let change = match &form {
                            Some(name) => Change::Form { name, previous: previous.as_deref() },
                            None if is_locale => Change::Locale,
                            None => Change::Theme { mix: &mix },
                        };
                        match apply_change(ui, change, &now) {
                            Ok(done) => aylis_core::stderr_line!("aylis: {done} ({file})"),
                            Err(e) => aylis_core::stderr_line!("aylis: {file}: {e}"),
                        }
                    });
                }
            }
        })
        .expect("thread do hot reload");
}

/// O que mudou no projeto em desenvolvimento (para [`apply_change`]).
#[derive(Clone, Copy, Debug)]
pub enum Change<'a> {
    /// O layout de um form (`previous`: o texto anterior, para mudar só o que mudou).
    Form { name: &'a str, previous: Option<&'a str> },
    /// O tema (com as misturas por classe do projeto).
    Theme { mix: &'a std::collections::BTreeMap<String, String> },
    /// As traduções do idioma atual.
    Locale,
}

/// Aplica no app aberto um arquivo do projeto que mudou (o hot reload: lido do disco no
/// desktop, ou mandado pela IDE no Android e no navegador). Devolve o que foi feito.
pub fn apply_change(ui: &mut Ui, change: Change<'_>, text: &str) -> Result<String, String> {
    let done = match change {
        Change::Form { name, previous } => {
            let Some(form) = ui.roots().iter().copied().find(|&r| ui.name(r).is_some_and(|n| n.eq_ignore_ascii_case(name))) else {
                // Ainda não criado: quando for, já lê o arquivo novo (no disco).
                return Ok(format!("{name} ainda não está aberto"));
            };
            let r = aylis_json::reload(ui, form, text, previous).map_err(|e| format!("{e} (o form continua como estava)"))?;
            format!("{name} recarregado ({} propriedade(s), {} novo(s), {} removido(s))", r.updated, r.created, r.removed)
        }
        Change::Theme { mix } => {
            let mut theme = aylis_json::load_theme(text).map_err(|e| format!("tema: {e}"))?;
            for (class, name) in mix {
                theme.mix(class, name);
            }
            ui.set_theme(theme);
            "tema recarregado".to_string()
        }
        Change::Locale => {
            ui.set_translations(i18n::parse(text));
            ui.retranslate();
            "traduções recarregadas".to_string()
        }
    };
    ui.request_repaint();
    Ok(done)
}
