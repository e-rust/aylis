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

/// Como o quadro desenhado chega à janela (no Windows, com o Direct2D; o `"present"` do
/// `.aylisproj`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PresentMode {
    /// A swap chain ligada à janela, como nos jogos: em tela cheia o quadro vai direto ao
    /// monitor (a menor latência) e pode trocar sem esperar o monitor (tearing). O padrão.
    #[default]
    Window,
    /// DirectComposition: o quadro vai para uma camada que o compositor do Windows (DWM)
    /// junta à janela, como nos navegadores (sempre composto, sem tearing). Não tira o aviso
    /// do Radeon Software em tela cheia (conferido em 04/11/2026).
    Composition,
}

impl PresentMode {
    /// Lê como o `.aylisproj` e o `AYLIS_PRESENT` gravam (vazio ou desconhecido: o padrão).
    pub fn parse(text: &str) -> PresentMode {
        match text.trim().to_lowercase().as_str() {
            "composition" | "composicao" | "composição" => PresentMode::Composition,
            _ => PresentMode::Window,
        }
    }

    /// O texto que [`PresentMode::parse`] lê de volta (vazio no padrão).
    pub fn name(self) -> &'static str {
        match self {
            PresentMode::Window => "",
            PresentMode::Composition => "composition",
        }
    }
}

static PRESENT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Grava o modo de apresentar (a plataforma faz isso ao abrir, com o do projeto).
pub fn set_present(mode: PresentMode) {
    PRESENT.store(mode == PresentMode::Composition, std::sync::atomic::Ordering::Relaxed);
}

/// O modo de apresentar em vigor.
pub fn present() -> PresentMode {
    if PRESENT.load(std::sync::atomic::Ordering::Relaxed) { PresentMode::Composition } else { PresentMode::Window }
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
