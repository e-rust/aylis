//! Qual placa de vídeo usar quando a máquina tem mais de uma (a integrada e a dedicada de um
//! notebook, ou duas placas num desktop). Vale para o processo inteiro: o Direct2D, o 3D e o
//! OpenGL do Linux (pelo Mesa) leem a escolha ao abrir a GPU.
//!
//! O projeto guarda a escolha (`"gpu"` no `.aylisproj`); com os diagnósticos ligados (exemplos,
//! testes e o app com `dev`), `AYLIS_GPU` troca na hora: `alto-desempenho`, `economia`, `monitor` ou parte do nome da placa (`AYLIS_GPU=radeon`).
//!
//! Cada form pode pedir outra (a propriedade `gpu` do `Form`; vazia: a do projeto). Com
//! `monitor`, a janela desenha na placa ligada ao monitor em que está e troca de placa quando
//! passa para um monitor da outra: sem isso, o Windows copia cada quadro de uma placa para a
//! outra, e a janela trava (veja `docs/gpu-por-monitor.md`).

use std::sync::{Mutex, OnceLock};

use crate::ComponentId;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum GpuChoice {
    /// A que o sistema escolher (a padrão).
    #[default]
    Auto,
    /// A mais rápida (a dedicada).
    HighPerformance,
    /// A que gasta menos energia (a integrada).
    LowPower,
    /// A placa do monitor em que a janela está, trocando quando ela muda de monitor (no
    /// Windows e no Linux com Wayland; nas outras plataformas, a do sistema).
    FollowMonitor,
    /// A primeira cujo nome contém este texto (sem diferenciar maiúsculas).
    Named(String),
}

impl GpuChoice {
    /// Lê a escolha como o `.aylisproj` e o `AYLIS_GPU` gravam.
    pub fn parse(text: &str) -> GpuChoice {
        match text.trim().to_lowercase().as_str() {
            "" | "auto" | "automatica" | "automática" => GpuChoice::Auto,
            "alto-desempenho" | "high-performance" | "alto" | "high" => GpuChoice::HighPerformance,
            "economia" | "low-power" | "baixo" | "low" => GpuChoice::LowPower,
            "monitor" | "seguir-monitor" | "follow-monitor" => GpuChoice::FollowMonitor,
            _ => GpuChoice::Named(text.trim().to_string()),
        }
    }

    /// O texto que [`GpuChoice::parse`] lê de volta (vazio na automática).
    pub fn to_text(&self) -> String {
        match self {
            GpuChoice::Auto => String::new(),
            GpuChoice::HighPerformance => "alto-desempenho".into(),
            GpuChoice::LowPower => "economia".into(),
            GpuChoice::FollowMonitor => "monitor".into(),
            GpuChoice::Named(name) => name.clone(),
        }
    }

    /// Se a placa `adapter` é a pedida pelo nome.
    pub fn matches_name(&self, adapter: &str) -> bool {
        match self {
            GpuChoice::Named(name) => adapter.to_lowercase().contains(&name.to_lowercase()),
            _ => false,
        }
    }
}

/// Como o quadro desenhado chega à janela (no Windows, com o Direct2D): o `"present"` do
/// `.aylisproj` e a propriedade `present` do form ([`present_for`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PresentMode {
    /// A swap chain ligada à janela, como nos jogos: em tela cheia o quadro vai direto ao
    /// monitor (a menor latência) e pode trocar sem esperar o monitor (tearing). O padrão.
    #[default]
    Window,
    /// DirectComposition: o quadro vai para uma camada que o compositor do Windows (DWM)
    /// junta à janela, como nos navegadores (sempre composto, sem tearing). Não tira o aviso
    /// do Radeon Software em tela cheia (conferido em 04/11/2026). O DWM ainda pode promover a
    /// camada a um plano de overlay (MPO).
    Composition,
    /// Sempre composta pelo DWM: a swap chain no modelo de cópia (*blit*), que nunca vai a um
    /// plano de overlay (MPO) nem a *independent flip*, que são só do modelo *flip*. Para forms
    /// em tela cheia que ficam por cima do resto (seleção de área, print, overlay): numa placa
    /// AMD com MPO, o cursor desenhado no plano de overlay congelava sobre um form assim
    /// (relatado pelo NoOBS em 15/11). Custa uma cópia a mais por quadro e não tem tearing.
    Composed,
    /// A composição (como `Composition`) com o alfa por pixel: a janela deixa ver o que está
    /// atrás onde o quadro não é opaco. Vem do form transparente ou com forma
    /// (`transparent`, `window_shape`), não do projeto.
    Transparent,
}

impl PresentMode {
    /// Lê como o `.aylisproj` e o `AYLIS_PRESENT` gravam (vazio ou desconhecido: o padrão).
    pub fn parse(text: &str) -> PresentMode {
        match text.trim().to_lowercase().as_str() {
            "composition" | "composicao" | "composição" => PresentMode::Composition,
            "composed" | "composto" => PresentMode::Composed,
            "transparent" | "transparente" => PresentMode::Transparent,
            _ => PresentMode::Window,
        }
    }

    /// O texto que [`PresentMode::parse`] lê de volta (vazio no padrão).
    pub fn name(self) -> &'static str {
        match self {
            PresentMode::Window => "",
            PresentMode::Composition => "composition",
            PresentMode::Composed => "composed",
            PresentMode::Transparent => "transparent",
        }
    }
}

static PRESENT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// Grava o modo de apresentar (a plataforma faz isso ao abrir, com o do projeto).
pub fn set_present(mode: PresentMode) {
    let code = match mode {
        PresentMode::Window => 0,
        PresentMode::Composition => 1,
        PresentMode::Composed => 2,
        PresentMode::Transparent => 3,
    };
    PRESENT.store(code, std::sync::atomic::Ordering::Relaxed);
}

/// O modo de apresentar do projeto.
pub fn present() -> PresentMode {
    match PRESENT.load(std::sync::atomic::Ordering::Relaxed) {
        1 => PresentMode::Composition,
        2 => PresentMode::Composed,
        3 => PresentMode::Transparent,
        _ => PresentMode::Window,
    }
}

/// O modo de apresentar de um form: a propriedade `present` dele ou, vazia, o do projeto.
pub fn present_for(form_text: &str) -> PresentMode {
    if form_text.trim().is_empty() || form_text.trim() == "project" { present() } else { PresentMode::parse(form_text) }
}

/// O `present` de um form: o do projeto ou um dos modos (`PresentMode` sem o `transparent`,
/// que vem do form transparente).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormPresent {
    #[default]
    Project,
    Window,
    Composition,
    Composed,
}

impl crate::PropEnum for FormPresent {
    const NAMES: &'static [&'static str] = &["project", "window", "composition", "composed"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Project, Self::Window, Self::Composition, Self::Composed].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
    /// Vazio é o do projeto (o padrão de antes, quando era texto).
    fn from_name(name: &str) -> Option<Self> {
        if name.trim().is_empty() {
            return Some(Self::Project);
        }
        Self::NAMES.iter().position(|n| n.eq_ignore_ascii_case(name.trim())).and_then(Self::from_index)
    }
}

/// Quem junta uma camada com `blend_mode` ao fundo no Direct2D (Windows): o efeito Blend do
/// próprio Direct2D ou o compositor do Aylis em Direct3D 11 (um shader com as fórmulas do
/// W3C). O resultado é o mesmo; o Direct3D é a base para o app que precisa de controle total
/// da placa. O `"blend_compositor"` do `.aylisproj`, a propriedade `blend_compositor` do form
/// ([`FormBlendCompositor`]) e o `RenderSettings` em execução; nos outros renderizadores, não
/// muda nada.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BlendCompositor {
    /// O efeito Blend do Direct2D. O padrão.
    #[default]
    Direct2d,
    /// O shader do Aylis em Direct3D 11, onde o alvo é uma textura da placa e os recortes são
    /// retângulos; no resto (a captura na CPU, recorte arredondado), o efeito do Direct2D.
    Direct3d,
}

impl crate::PropEnum for BlendCompositor {
    const NAMES: &'static [&'static str] = &["direct2d", "direct3d"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Direct2d, Self::Direct3d].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

static BLEND_COMPOSITOR: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// Grava o compositor da mistura do app (o do projeto ao abrir; o `RenderSettings` em execução).
pub fn set_blend_compositor(compositor: BlendCompositor) {
    BLEND_COMPOSITOR.store(compositor as u8, std::sync::atomic::Ordering::Relaxed);
}

/// O compositor da mistura do app.
pub fn blend_compositor() -> BlendCompositor {
    if BLEND_COMPOSITOR.load(std::sync::atomic::Ordering::Relaxed) == 1 { BlendCompositor::Direct3d } else { BlendCompositor::Direct2d }
}

/// O `blend_compositor` de um form: o do projeto ou um dos dois.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormBlendCompositor {
    #[default]
    Project,
    Direct2d,
    Direct3d,
}

impl FormBlendCompositor {
    /// O compositor próprio do form (`None`: o do projeto).
    pub fn own(self) -> Option<BlendCompositor> {
        match self {
            Self::Project => None,
            Self::Direct2d => Some(BlendCompositor::Direct2d),
            Self::Direct3d => Some(BlendCompositor::Direct3d),
        }
    }
}

impl crate::PropEnum for FormBlendCompositor {
    const NAMES: &'static [&'static str] = &["project", "direct2d", "direct3d"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Project, Self::Direct2d, Self::Direct3d].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

static CHOICE: Mutex<GpuChoice> = Mutex::new(GpuChoice::Auto);

/// Grava a escolha (a plataforma faz isso ao abrir, com a do projeto).
pub fn set_choice(choice: GpuChoice) {
    if let Ok(mut c) = CHOICE.lock() {
        *c = choice;
    }
}

/// A escolha em vigor (a do projeto; com os diagnósticos, a plataforma aceita `AYLIS_GPU`).
pub fn choice() -> GpuChoice {
    CHOICE.lock().map(|c| c.clone()).unwrap_or_default()
}

/// A escolha de um form: a propriedade `gpu` dele (`form_text`) ou, vazia, a do projeto.
pub fn choice_for(form_text: &str) -> GpuChoice {
    if form_text.trim().is_empty() { choice() } else { GpuChoice::parse(form_text) }
}

/// Quem lista as placas (a plataforma põe ao abrir) e a lista, perguntada só na primeira vez
/// que alguém pede: as placas não mudam com o app aberto.
static ADAPTER_LIST: OnceLock<fn() -> Vec<String>> = OnceLock::new();
static ADAPTERS: OnceLock<Vec<String>> = OnceLock::new();

/// A plataforma diz como listar as placas de vídeo (no Windows, pelo DXGI).
pub fn set_adapter_list(list: fn() -> Vec<String>) {
    let _ = ADAPTER_LIST.set(list);
}

/// As placas de vídeo da máquina, pelo nome (vazio onde não se sabe).
pub fn adapters() -> &'static [String] {
    ADAPTERS.get_or_init(|| ADAPTER_LIST.get().map(|list| list()).unwrap_or_default())
}

impl crate::Ui {

    /// A placa em que a janela do form desenha agora (`None`: não se sabe, como na CPU).
    pub fn form_gpu(&self, form: ComponentId) -> Option<&str> {
        self.form_gpus.get(&self.root_of(form)).map(String::as_str)
    }

    /// A plataforma informa a placa da janela do form. Se mudou (não na primeira vez), dispara
    /// o `on_gpu_change` do form com o nome da placa nova.
    pub fn set_form_gpu(&mut self, form: ComponentId, adapter: Option<String>) {
        let old = match adapter.clone() {
            Some(a) => self.form_gpus.insert(form, a),
            None => self.form_gpus.remove(&form),
        };
        if let (Some(old), Some(new)) = (old, adapter)
            && old != new
        {
            self.fire(form, "on_gpu_change", &crate::EventArgs::Text(new));
        }
    }
}

thread_local! {
    static BEFORE_FLUSH: std::cell::RefCell<Vec<Box<dyn FnOnce()>>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Pede que `f` rode no fim do quadro, logo antes de o canvas na GPU executar o que gravou
/// (o `flush` do Skia): quem desenha por fora no mesmo contexto (o 3D) grava os comandos de
/// todos os seus controles durante a pintura e envia tudo de uma vez ali, em vez de um envio
/// por controle. Roda uma vez; peça de novo no quadro seguinte.
pub fn before_flush(f: impl FnOnce() + 'static) {
    let _ = BEFORE_FLUSH.try_with(|list| list.borrow_mut().push(Box::new(f)));
}

/// Roda o que foi pedido por [`before_flush`] (o canvas chama antes do flush). `true` se algo
/// rodou: o estado da GPU mudou e o canvas precisa refazer o dele.
pub fn run_before_flush() -> bool {
    let pending = BEFORE_FLUSH.try_with(|list| std::mem::take(&mut *list.borrow_mut())).unwrap_or_default();
    let ran = !pending.is_empty();
    for f in pending {
        f();
    }
    ran
}
