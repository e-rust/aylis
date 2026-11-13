//! [`Ui`]: dono de todos os componentes (arena), da árvore pai/filhos, dos nomes, das
//! propriedades por nome, dos handlers de evento e do tema.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt::Write as _;

use crate::anim::Running;
use crate::canvas::{Canvas, TextStyle};
use crate::component::{ColorOverride, Component, ComponentId, ControlBase};
use crate::error::{Error, Result};
use crate::event::{EventArgs, EventCtx};
use crate::component::ControlState;
use crate::geometry::{Bounds, RectF, SizeF};
use crate::input::InputState;
use crate::paint::{PaintCtx, Visual};
use crate::platform::Services;
use crate::prop::{PropInfo, PropKind, PropValue};
use crate::registry::{ClassInfo, ClassRegistry, create_default};
use crate::theme::Theme;

/// As propriedades do controle que só mudam a pintura (as cores também): mudá-las não refaz o
/// layout ([`Ui::set_prop`]). Os componentes acrescentam as deles
/// ([`Component::paint_only_property`]).
const PAINT_ONLY: [&str; 9] = ["opacity", "rotation", "translate_x", "translate_y", "cursor", "hint", "shadow", "hit_test", "cache_layer"];

/// Eventos entregues por chamada de [`Ui::process_events`]; o resto espera a próxima volta.
pub const EVENTS_PER_ROUND: usize = 10_000;
/// Repetições seguidas do mesmo evento do mesmo componente que viram aviso de possível laço.
pub const LOOP_WARNING: usize = 1_000;
/// Profundidade máxima da árvore de componentes (o padrão; o projeto muda com
/// [`Ui::set_limits`]): o layout e a pintura são recursivos, e uma cadeia de painéis criada em
/// laço (pelo protocolo, por exemplo) estouraria a pilha.
pub const MAX_TREE_DEPTH: usize = 256;

/// Eventos aninhados (um handler disparando outro) até o erro de recursão (o padrão; o
/// projeto muda com [`Ui::set_limits`]).
pub const MAX_EVENT_NESTING: usize = 64;

/// Acima destes limites, a pilha do programa pode estourar antes (a IDE avisa).
pub const SAFE_TREE_DEPTH: usize = 1024;
pub const SAFE_EVENT_NESTING: usize = 256;

/// Handler de evento: recebe o `Ui`, o componente que disparou (o `Sender`) e os argumentos.
/// É `Fn` (estado mutável num `Cell`/`RefCell`) para poder ser chamado de novo enquanto roda.
pub type HandlerFn = dyn Fn(&mut Ui, ComponentId, &EventArgs);

/// Recebe os eventos ligados a um nome de handler que não foi registrado em Rust
/// (`sender`, nome do evento, nome do handler, argumentos). É o ponto de saída para a
/// C ABI e para o protocolo remoto.
pub type EventSinkFn = dyn Fn(&mut Ui, ComponentId, &str, &str, &EventArgs);

/// Pintura personalizada de uma instância (como o `OnPaint`): recebe o canvas, o
/// contexto e a pintura padrão do controle, que pode ser chamada ou não.
pub type PaintHookFn = dyn Fn(&mut dyn Canvas, &PaintCtx<'_>, &dyn Fn(&mut dyn Canvas, &PaintCtx<'_>));

#[derive(Clone)]
pub(crate) enum Handler {
    Closure(std::rc::Rc<HandlerFn>),
    Named(String),
}

pub(crate) struct Entry {
    /// `None` enquanto um método do próprio componente está executando.
    pub(crate) component: Option<Box<dyn Component>>,
    pub(crate) name: String,
    /// Escopo de nomes (como o `Owner` do Delphi): os nomes dos componentes de dentro só
    /// precisam ser únicos aqui. Todo form é um escopo, inclusive embutido em outro.
    pub(crate) scope: bool,
    pub(crate) parent: Option<ComponentId>,
    pub(crate) children: Vec<ComponentId>,
    pub(crate) events: Vec<(String, Handler)>,
    pub(crate) paint_hook: Option<Box<PaintHookFn>>,
    /// Último estilo pintado e transição em andamento (interior: a pintura usa `&self`).
    pub(crate) visual: RefCell<Visual>,
    /// As propriedades ligadas a dados.
    pub(crate) bindings: Vec<crate::bindings::Binding>,
}

struct Slot {
    generation: u32,
    entry: Option<Entry>,
}

/// Ver [`Ui::set_event_trace`].
pub type TraceFn = dyn Fn(&Ui, ComponentId, &str, &str);

/// De onde vem o tema de um controle ([`Ui::theme_source`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThemeSource {
    /// O tema do app (o do projeto).
    App,
    /// A propriedade `theme` do próprio controle.
    Own,
    /// A propriedade `theme` de um ancestral.
    Inherited(ComponentId),
    /// A mistura por classe do tema ([`Theme::mix`]): a classe indicada usa outro tema.
    Class(String),
}

/// Mensagem do designer: o documento mudou ([`Ui::design_changed`]).
pub struct DesignChanged;

/// A árvore de componentes e tudo que está ligado a ela.
///
/// ```
/// use aylis_core::*;
///
/// #[derive(Default)]
/// struct Block { base: ControlBase }
///
/// impl Component for Block {
///     fn class_name(&self) -> &'static str { "Block" }
///     fn control(&self) -> Option<&ControlBase> { Some(&self.base) }
///     fn control_mut(&mut self) -> Option<&mut ControlBase> { Some(&mut self.base) }
/// }
///
/// let mut ui = Ui::new();
/// let form = ui.add_root(Block::default());
/// ui.control_mut(form).unwrap().set_size(400.0, 300.0);
///
/// let top = ui.add(Block::default(), form);
/// ui.set_prop(top, "align", "top").unwrap();
/// ui.set_prop(top, "height", 40).unwrap();
///
/// ui.layout();
/// assert_eq!(ui.bounds(top), Some(RectF::new(0.0, 0.0, 400.0, 40.0)));
/// ```
pub struct Ui {
    slots: Vec<Slot>,
    free_slots: Vec<u32>,
    roots: Vec<ComponentId>,
    registry: ClassRegistry,
    /// Os handlers por nome: form → nome do handler → closure (os sem form ficam em `""`):
    /// achar o do form não monta texto a cada evento.
    handlers: HashMap<String, HashMap<String, std::rc::Rc<HandlerFn>>>,
    event_sink: Option<std::rc::Rc<EventSinkFn>>,
    /// Quantos handlers estão rodando um dentro do outro.
    event_depth: usize,
    /// Acima de zero, o `set_prop` não dispara o `on_change` do valor (carregando um form,
    /// veja [`Ui::without_change_events`]).
    quiet_changes: u32,
    /// Os eventos cujos handlers estão rodando agora (o de cima é o mais recente): o aviso de
    /// evento adiado, em desenvolvimento, pula os automáticos (`on_resize`...).
    pub(crate) running_events: Vec<String>,
    /// Rastro de eventos (depuração): chamado a cada evento com handler.
    trace: Option<std::rc::Rc<TraceFn>>,
    /// Quem ouve a atividade do app ([`crate::activity`]).
    observers: crate::activity::Observers,
    /// Quem grava de volta e quem mostra as ligações a dados (ver `bindings.rs`).
    pub(crate) binding_sink: Option<crate::bindings::BindingSink>,
    pub(crate) binding_refresh: Option<crate::bindings::BindingSink>,
    /// As vistas por largura de cada form (veja `views.rs`).
    pub(crate) views: HashMap<ComponentId, crate::views::ViewState>,
    pending: Vec<(ComponentId, String, EventArgs)>,
    /// O último evento entregue e quantas vezes seguidas (para avisar de um possível laço).
    repeat: Option<(ComponentId, String, usize)>,
    /// Compartilhado (`Arc`): o tema padrão é o mesmo em todos os `Ui` (cada tema tem ~0,2 MB,
    /// e a IDE abre um `Ui` por aba: com os 1009 forms do ERP abertos, 600 MB só de cópias); quem
    /// muda copia antes (`Arc::make_mut`).
    pub(crate) theme: std::sync::Arc<Theme>,
    /// Biblioteca de temas, pelo nome (para a propriedade `theme` e para [`Theme::mix`]).
    themes: Vec<std::sync::Arc<Theme>>,
    /// A paleta do projeto: entra em todo tema usado (veja [`crate::palette`]).
    palette: crate::palette::Palette,
    /// Como a roda rola (passo por dente, deslize).
    scroll_options: crate::ScrollOptions,
    /// O zoom do app inteiro (1 = normal); cada form ainda multiplica pelo `zoom` dele.
    zoom: f32,
    /// O zoom mudou: a plataforma refaz o tamanho dos forms e repinta.
    zoom_changed: bool,
    /// Ctrl + `+`/`-`/`0` e Ctrl + roda mudam o zoom do app.
    zoom_keys: bool,
    /// O painel de memória ([`crate::memory_panel`]): `None` indisponível, `Some(visível)`.
    pub(crate) memory_panel: Option<bool>,
    /// As fontes do projeto por nome (`titulo` → `Montserrat`): valem no `font_family` dos
    /// componentes e no `family` dos estilos ([`resolve_font`](Self::resolve_font)).
    font_names: Vec<(String, String)>,
    /// As propriedades de cor escritas com um nome: (controle, propriedade) → o nome e a cor
    /// que ele deu. Trocar o tema recalcula; o form salvo guarda o nome. Se a cor do
    /// componente já não é a que o nome deu (alguém a trocou direto, no campo), o nome vale
    /// mais nada ([`Ui::color_name`]).
    color_names: HashMap<(ComponentId, String), (String, crate::color::Color)>,
    /// Ícones registrados pelo app (os do aylis ficam em [`crate::icons`]).
    icons: HashMap<String, crate::icons::Icon>,
    /// Estado do ponteiro e do foco, por form.
    pub(crate) input: HashMap<ComponentId, InputState>,
    /// O que o handler de `on_drag_start` pediu para arrastar (dados, rótulo).
    pub(crate) drag_request: Option<(String, String)>,
    /// O que o handler de `on_drag_start` pediu para acompanhar o cursor ([`Ui::set_drag_image`]).
    pub(crate) drag_visual_request: Option<crate::DragVisual>,
    /// A plataforma mostra a imagem do arrasto numa janela própria (fora da janela também):
    /// o form não a pinta.
    pub(crate) native_drag_image: bool,
    /// O handler de `on_drag_over` recusou o destino.
    pub(crate) drop_rejected: bool,
    /// Onde o mouse está na tela durante um arrasto (pixels físicos) e a escala da janela
    /// dele: a plataforma informa ([`Ui::set_screen_pointer`]).
    pub(crate) screen_pointer: Option<crate::input::ScreenPointer>,
    pub(crate) animations: Vec<Running>,
    /// As animações `appear` que ainda esperam aparecer (`None`: refazer a lista).
    pub(crate) appear_waiting: Option<Vec<ComponentId>>,
    /// Alguma transição de estilo ainda está em andamento (marcado durante a pintura).
    pub(crate) style_animating: Cell<bool>,
    /// Os temas com as cores próprias de controles (pelo tema de base e as cores): não
    /// copiar o tema a cada quadro.
    pub(crate) recolored: RefCell<HashMap<(String, ColorOverride), std::rc::Rc<Theme>>>,
    /// Próximo horário (em [`Ui::time`]) em que alguém pediu um frame.
    pub(crate) wake_at: Cell<Option<f64>>,
    /// O que cada form pediu na última pintura dele (transição em andamento, próximo
    /// frame): cada janela pinta em separado, e uma não apaga o pedido da outra.
    pub(crate) frame_requests: std::cell::RefCell<HashMap<ComponentId, (bool, Option<f64>)>>,
    /// Os componentes que pediram os próximos quadros ([`Component::frame`]).
    pub(crate) frame_hooks: Vec<ComponentId>,
    /// O que os controles pediram para desenhar por cima de tudo neste quadro (o balão do
    /// tempo da barra de mídia...): a posição no form e o desenho.
    pub(crate) overlays: std::cell::RefCell<Vec<(crate::geometry::PointF, crate::pages::Page)>>,
    /// Foco pedido de dentro de um método de evento ([`EventCtx::request_focus`]).
    pub(crate) pending_focus: Option<ComponentId>,
    close_requested: bool,
    /// O tratamento de erros (veja `exception.rs`).
    pub(crate) exceptions: crate::exception::Exceptions,
    /// Os limites de segurança: profundidade da árvore e eventos aninhados ([`Ui::set_limits`]).
    pub(crate) limits: (usize, usize),
    /// Quem aponta para quem (veja `references.rs`); `None` = montar de novo.
    pub(crate) references: RefCell<Option<crate::references::Index>>,
    /// Os `EventInterceptor` (veja `intercept.rs`).
    pub(crate) interceptors: crate::intercept::Interceptors,
    /// O índice de nomes (veja `names.rs`); `None` = montar de novo.
    pub(crate) names: RefCell<crate::names::NameIndex>,
    /// As tarefas `async` na thread da tela (veja `tasks.rs`).
    pub(crate) tasks: crate::tasks::Tasks,
    /// As instâncias dos forms (veja `instances.rs`).
    pub(crate) instances: crate::instances::Instances,
    /// Forms abertos como janelas, do fundo para o topo (ver `forms.rs`).
    pub(crate) open_forms: Vec<crate::forms::OpenForm>,
    /// Onde está (ou deve abrir) a janela de cada form (ver `forms.rs`).
    pub(crate) placements: std::collections::HashMap<ComponentId, crate::forms::WindowPlacement>,
    /// Os monitores, quantas vezes a lista mudou e o monitor de cada form ([`crate::monitors`]).
    pub(crate) monitors: Vec<crate::monitors::Monitor>,
    pub(crate) monitors_version: u64,
    pub(crate) form_monitors: std::collections::HashMap<ComponentId, usize>,
    /// Onde o último arrasto foi solto, no destino (`Ui::drop_point`).
    pub(crate) drop_point: Option<crate::geometry::PointF>,
    /// Os forms com a janela minimizada (`monitors.rs`).
    pub(crate) minimized_forms: std::collections::HashSet<ComponentId>,
    /// A placa de vídeo de cada janela (veja `gpu.rs`).
    pub(crate) form_gpus: std::collections::HashMap<ComponentId, String>,
    pub(crate) main_form: Option<ComponentId>,
    /// Os eventos-pergunta em andamento e os adiados (`question.rs`).
    pub(crate) questions: crate::question::Questions,
    /// O backend em outro processo ocupado (`busy.rs`).
    pub(crate) busy: crate::busy::Busy,
    /// Quem mede e desenha o texto (`textcore.rs`).
    pub(crate) text_mode: crate::textcore::TextMode,
    /// O estado que os outros crates guardam neste `Ui` (um por tipo: veja [`Ui::extension`]).
    extensions: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
    /// Pedidos às janelas (barra de título própria), para a plataforma.
    pub(crate) window_commands: Vec<(ComponentId, crate::window::WindowCommand)>,
    /// O botão maximizar de cada form com barra própria: quem o desenha e onde, no lugar dele
    /// (o sistema mostra ali o seu menu de layouts; [`Ui::maximize_button`]).
    pub(crate) maximize_buttons: HashMap<ComponentId, (ComponentId, crate::geometry::RectF)>,
    /// Quadros desenhados (para o FPS) e o limite.
    pub(crate) frames: crate::frames::FrameClock,
    /// Transições de página e de form em andamento.
    pub(crate) transitions: Vec<crate::transition::Running>,
    /// O form que uma transição de saída está fechando agora (sem repetir a transição).
    pub(crate) closing_now: Option<ComponentId>,
    /// Os forms se empilham numa superfície só (celular, navegador): abrir e fechar usam a
    /// `transition` do form.
    pub(crate) stacked_forms: bool,
    pub(crate) activate: Option<ComponentId>,
    /// Forms modais com resultado definido durante um evento: fecham quando o evento
    /// termina (o handler ainda pode voltar o resultado para `None`).
    pub(crate) pending_closes: Vec<ComponentId>,
    /// Quantos métodos de componente estão executando (um dentro do outro).
    pub(crate) call_depth: u32,
    pub(crate) design_mode: bool,
    /// O atraso da dica escolhido pelo app (o `HintPause` do Delphi); `None`: o do tema.
    hint_delay: Option<f64>,
    /// Pintando uma foto (o `snapshot`): sem o mouse em cima, o aperto e o foco (o cursor de
    /// texto piscando e o anel), que são do momento e não da tela (B03 do Tabernáculo).
    pub(crate) still_picture: bool,
    /// No designer, os componentes escondidos (`visible = false`) que aparecem: os
    /// selecionados e os que contêm um selecionado ([`Ui::set_design_reveal`]).
    design_reveal: Vec<ComponentId>,
    /// No designer, todos os escondidos aparecem (translúcidos).
    design_show_hidden: bool,
    /// Os `set_timeout` pendentes e o último número dado.
    pub(crate) timeouts: Vec<crate::timers::Timeout>,
    pub(crate) next_timeout: u64,
    /// Forms cuja janela nasce escondida ([`Ui::set_start_hidden`]).
    pub(crate) start_hidden: std::collections::HashSet<ComponentId>,
    /// Interface espelhada (da direita para a esquerda: árabe, hebraico).
    pub(crate) rtl: bool,
    /// Funções agendadas por outras threads (ver `post.rs`).
    pub(crate) posted: std::sync::Arc<crate::post::PostQueue>,
    /// Timers agendados (ver `timers.rs`).
    pub(crate) timers: Vec<crate::timers::TimerEntry>,
    /// Traduções (ver `i18n.rs`).
    pub(crate) translations: crate::i18n::Translations,
    services: Services,
    pub(crate) time: f64,
    pub(crate) layout_dirty: bool,
    /// Os forms com o layout a refazer ([`Ui::invalidate_form`]); `layout_dirty` vale para todos.
    pub(crate) dirty_forms: std::collections::HashSet<ComponentId>,
    pub(crate) repaint_requested: bool,
    /// Quantas vezes a janela inteira foi marcada para repintar ([`Ui::request_repaint`]) e,
    /// de cada form, até qual delas ele já pintou ([`Ui::take_damage`]).
    pub(crate) damage_epoch: u64,
    pub(crate) painted_epoch: HashMap<ComponentId, u64>,
    /// A extensão do desenho do controle sendo pintado ([`crate::ink`]) e se algum desenhou
    /// fora da área de uma pintura por parte (o próximo quadro repinta tudo).
    pub(crate) ink: RefCell<Option<RectF>>,
    pub(crate) spilled: Cell<bool>,
    pub(crate) paint_area: Cell<Option<RectF>>,
    /// Os controles que desenharam por cima de tudo no último quadro (`PaintCtx::overlay`):
    /// o desenho deles sai da área deles.
    pub(crate) overlay_users: std::cell::RefCell<std::collections::HashSet<ComponentId>>,
    /// Os controles cujo desenho mudou (`ctx.repaint()`, o próximo quadro pedido na pintura, a
    /// animação do estilo): o próximo quadro repinta a área deles ([`Ui::take_damage`]).
    pub(crate) frame_ids: std::cell::RefCell<Vec<ComponentId>>,
    /// Os que só mudaram de lugar ou de transparência (`translate`, `opacity`): a área deles
    /// é repintada, mas o desenho de dentro é o mesmo ([`Ui::damage_placement`]).
    pub(crate) placed_ids: std::cell::RefCell<Vec<ComponentId>>,
    /// As partes que mudaram de um controle (nas coordenadas dele): [`Ui::damage_part`].
    pub(crate) part_ids: std::cell::RefCell<Vec<(ComponentId, RectF)>>,
    /// O conteúdo que andou (a rolagem): [`Ui::scroll_damage`].
    pub(crate) scroll_moves: std::cell::RefCell<Vec<(ComponentId, RectF, crate::PointF)>>,
    /// Muda a cada pintura inteira: as camadas guardadas (`cache_layer`) só valem entre
    /// pinturas por parte, em que o `Ui` sabe o que mudou.
    pub(crate) layer_epoch: std::cell::Cell<u64>,
    /// As camadas guardadas que deixaram de valer fora da pintura (a transição acabou, o
    /// controle foi liberado): a próxima pintura as solta do renderizador (`forget_cached`).
    pub(crate) dead_layers: std::cell::RefCell<Vec<u64>>,
    /// As camadas automáticas valem no form sendo pintado (a propriedade `layer_cache` dele,
    /// ou o padrão do projeto): lido no começo de cada pintura.
    pub(crate) auto_layers: std::cell::Cell<bool>,
    /// Quantos controles a última pintura desenhou (os que passaram do recorte das áreas):
    /// a medida do trabalho de um quadro que não depende da máquina ([`Ui::last_paint_count`]).
    pub(crate) paint_count: std::cell::Cell<usize>,
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}

/// O "Aylis Light" ou o "Aylis Dark" de fábrica, um só para todos os `Ui` do processo.
fn shared_theme(dark: bool) -> std::sync::Arc<Theme> {
    static LIGHT: std::sync::OnceLock<std::sync::Arc<Theme>> = std::sync::OnceLock::new();
    static DARK: std::sync::OnceLock<std::sync::Arc<Theme>> = std::sync::OnceLock::new();
    let cell = if dark { &DARK } else { &LIGHT };
    cell.get_or_init(|| std::sync::Arc::new(if dark { Theme::dark() } else { Theme::light() })).clone()
}

impl Ui {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_slots: Vec::new(),
            roots: Vec::new(),
            registry: ClassRegistry::default(),
            handlers: HashMap::new(),
            event_sink: None,
            event_depth: 0,
            quiet_changes: 0,
            running_events: Vec::new(),
            trace: None,
            observers: Default::default(),
            binding_sink: None,
            binding_refresh: None,
            views: HashMap::new(),
            pending: Vec::new(),
            repeat: None,
            theme: shared_theme(false),
            themes: vec![shared_theme(false), shared_theme(true)],
            palette: Vec::new(),
            scroll_options: crate::ScrollOptions::default(),
            zoom: 1.0,
            zoom_changed: false,
            zoom_keys: false,
            memory_panel: None,
            font_names: Vec::new(),
            color_names: HashMap::new(),
            icons: HashMap::new(),
            input: HashMap::new(),
            drag_request: None,
            drag_visual_request: None,
            native_drag_image: false,
            drop_rejected: false,
            screen_pointer: None,
            animations: Vec::new(),
            appear_waiting: None,
            style_animating: Cell::new(false),
            recolored: RefCell::new(HashMap::new()),
            wake_at: Cell::new(None),
            frame_requests: Default::default(),
            frame_hooks: Vec::new(),
            overlays: Default::default(),
            pending_focus: None,
            close_requested: false,
            exceptions: Default::default(),
            limits: (MAX_TREE_DEPTH, MAX_EVENT_NESTING),
            references: RefCell::new(None),
            interceptors: Default::default(),
            names: RefCell::default(),
            tasks: Default::default(),
            instances: Default::default(),
            open_forms: Vec::new(),
            placements: std::collections::HashMap::new(),
            monitors: Vec::new(),
            monitors_version: 0,
            form_monitors: std::collections::HashMap::new(),
            minimized_forms: std::collections::HashSet::new(),
            drop_point: None,
            form_gpus: std::collections::HashMap::new(),
            main_form: None,
            questions: Default::default(),
            busy: Default::default(),
            text_mode: Default::default(),
            extensions: Default::default(),
            frames: Default::default(),
            window_commands: Vec::new(),
            maximize_buttons: HashMap::new(),
            transitions: Vec::new(),
            closing_now: None,
            stacked_forms: false,
            activate: None,
            pending_closes: Vec::new(),
            call_depth: 0,
            design_mode: false,
            hint_delay: None,
            still_picture: false,
            design_reveal: Vec::new(),
            design_show_hidden: false,
            timeouts: Vec::new(),
            next_timeout: 0,
            start_hidden: Default::default(),
            rtl: false,
            posted: Default::default(),
            timers: Vec::new(),
            translations: Default::default(),
            services: Services::default(),
            time: 0.0,
            layout_dirty: false,
            dirty_forms: Default::default(),
            repaint_requested: false,
            damage_epoch: 0,
            painted_epoch: HashMap::new(),
            ink: Default::default(),
            spilled: Default::default(),
            paint_area: Default::default(),
            overlay_users: Default::default(),
            frame_ids: Default::default(),
            placed_ids: Default::default(),
            part_ids: Default::default(),
            scroll_moves: Default::default(),
            layer_epoch: Default::default(),
            dead_layers: Default::default(),
            auto_layers: std::cell::Cell::new(true),
            paint_count: Default::default(),
        }
    }

    // =====================================================================
    // Arena
    // =====================================================================

    pub(crate) fn entry(&self, id: ComponentId) -> Option<&Entry> {
        let slot = self.slots.get(id.index as usize)?;
        if slot.generation != id.generation {
            return None;
        }
        slot.entry.as_ref()
    }

    pub(crate) fn entry_mut(&mut self, id: ComponentId) -> Option<&mut Entry> {
        let slot = self.slots.get_mut(id.index as usize)?;
        if slot.generation != id.generation {
            return None;
        }
        slot.entry.as_mut()
    }

    fn alloc(&mut self, entry: Entry) -> ComponentId {
        // A árvore mudou: a lista das animações `appear` é refeita.
        self.appear_waiting = None;
        if let Some(index) = self.free_slots.pop() {
            let slot = &mut self.slots[index as usize];
            slot.entry = Some(entry);
            ComponentId { index, generation: slot.generation }
        } else {
            self.slots.push(Slot { generation: 0, entry: Some(entry) });
            ComponentId { index: (self.slots.len() - 1) as u32, generation: 0 }
        }
    }

    /// `ControlBase` sem marcar nada como alterado (uso interno do layout e da pintura).
    pub(crate) fn cb(&self, id: ComponentId) -> Option<&ControlBase> {
        self.entry(id)?.component.as_deref()?.control()
    }

    pub(crate) fn cb_mut(&mut self, id: ComponentId) -> Option<&mut ControlBase> {
        self.entry_mut(id)?.component.as_deref_mut()?.control_mut()
    }

    /// Marca que o layout de todos os forms precisa ser recalculado e a tela repintada.
    /// Sabendo o controle que mudou, [`Ui::invalidate_form`] refaz só o form dele.
    #[track_caller]
    pub fn invalidate(&mut self) {
        self.layout_dirty = true;
        self.request_repaint();
    }

    /// Marca que o layout do form de `id` precisa ser recalculado (os outros forms abertos
    /// ficam como estão) e que `id` mudou. A repintura é só de `id` e de quem o layout
    /// mudar de lugar, de tamanho ou de visibilidade ([`Ui::update_layout`]).
    pub fn invalidate_form(&mut self, id: ComponentId) {
        let root = self.root_of(id);
        self.dirty_forms.insert(root);
        self.damage(id);
    }

    /// Pede a repintura da janela inteira (sem saber o que mudou). Para só um controle,
    /// [`Ui::damage`].
    #[track_caller]
    pub fn request_repaint(&mut self) {
        self.repaint_requested = true;
        self.damage_epoch += 1;
    }

    /// O layout está marcado para ser refeito na próxima pintura.
    pub fn needs_layout(&self) -> bool {
        self.layout_dirty || !self.dirty_forms.is_empty()
    }

    /// Há algo novo para pintar (mudança, layout pendente ou animação em andamento).
    pub fn needs_repaint(&self) -> bool {
        self.repaint_requested || self.needs_layout() || self.is_animating()
    }

    // =====================================================================
    // Criação e destruição
    // =====================================================================

    /// Cria `component` como filho de `parent` (como `Create` + `Parent :=`).
    ///
    /// # Panics
    /// Se `parent` não existir. Use [`add_boxed`](Self::add_boxed) para receber um erro.
    pub fn add<T: Component>(&mut self, component: T, parent: ComponentId) -> ComponentId {
        self.add_boxed(Box::new(component), Some(parent))
            .expect("aylis: Ui::add com pai inexistente ou árvore funda demais")
    }

    /// Cria um componente sem pai (um form, um data module...).
    pub fn add_root<T: Component>(&mut self, component: T) -> ComponentId {
        self.add_boxed(Box::new(component), None).expect("sem pai não há como falhar")
    }

    pub fn add_boxed(
        &mut self,
        component: Box<dyn Component>,
        parent: Option<ComponentId>,
    ) -> Result<ComponentId> {
        if let Some(p) = parent {
            if !self.contains(p) {
                return Err(Error::InvalidId);
            }
            if self.depth_of(p) + 1 > self.limits.0 {
                return Err(too_deep(self.limits.0));
            }
        }
        let id = self.alloc(Entry {
            component: Some(component),
            name: String::new(),
            scope: parent.is_none(),
            parent,
            children: Vec::new(),
            events: Vec::new(),
            paint_hook: None,
            visual: RefCell::default(),
            bindings: Vec::new(),
        });
        match parent {
            Some(p) => self.entry_mut(p).expect("verificado acima").children.push(id),
            None => self.roots.push(id),
        }
        self.invalidate_form(id);
        self.forget_references();
        self.sync_timer(id);
        // Entra sem nome: o índice de nomes não muda (o `set_name` avisa depois).
        if self.class_name(id) == Some(crate::intercept::INTERCEPTOR_CLASS) {
            self.interceptors.forget();
        }
        Ok(id)
    }

    /// Cria pelo nome de uma classe registrada (como `GetClass('TButton').Create`).
    ///
    /// Aceita também um subtipo, `"Classe.variante"` (`"Button.icon"`): cria a classe,
    /// aplica os valores iniciais do subtipo registrado e põe `style = variante`.
    pub fn create(&mut self, class: &str, parent: Option<ComponentId>) -> Result<ComponentId> {
        let (base, variant) = crate::registry::split_variant(class);
        let create = self
            .registry
            .get(base)
            .ok_or_else(|| Error::UnknownClass(base.to_string()))?
            .create;
        let id = self.add_boxed(create(), parent)?;
        if let Some(variant) = variant {
            self.apply_variant(id, base, variant)?;
        }
        Ok(id)
    }

    /// Aplica um subtipo a um componente já criado: valores iniciais (se registrado) e
    /// `style`. Uma variante que só existe no tema também vale.
    pub fn apply_variant(&mut self, id: ComponentId, class: &str, variant: &str) -> Result<()> {
        if let Some(info) = self.registry.variant(class, variant).cloned() {
            for (name, value) in info.props {
                self.set_prop_quiet(id, &name, value)?;
            }
        } else if self.theme.style(&format!("{class}.{variant}")).is_none() {
            return Err(Error::UnknownClass(format!("{class}.{variant}")));
        }
        if let Some(cb) = self.control_mut(id) {
            cb.set_style(variant);
        }
        Ok(())
    }

    /// Registra um subtipo de uma classe (aparece na paleta debaixo dela).
    pub fn register_variant(&mut self, class: &str, name: &str, caption: &str, props: Vec<(&str, PropValue)>) {
        self.registry.register_variant(crate::registry::VariantInfo {
            class: class.to_string(),
            name: name.to_string(),
            caption: caption.to_string(),
            props: props.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
        });
    }

    /// Os subtipos de uma classe: os registrados e os que só existem no tema atual
    /// (variantes criadas pelo usuário num tema JSON), sem repetir.
    pub fn variants_of(&self, class: &str) -> Vec<crate::registry::VariantInfo> {
        let mut out: Vec<crate::registry::VariantInfo> = self.registry.variants_of(class).cloned().collect();
        for name in self.theme.variants_of(class) {
            if !out.iter().any(|v| v.name.eq_ignore_ascii_case(&name)) {
                out.push(crate::registry::VariantInfo { class: class.to_string(), caption: name.clone(), name, props: Vec::new() });
            }
        }
        out
    }

    /// Libera o componente e todos os descendentes (como `Free`). Os demais componentes
    /// recebem [`Component::notification`] para cada um liberado.
    ///
    /// Devolve `false` se o id já não existia.
    pub fn free(&mut self, id: ComponentId) -> bool {
        self.free_many(&[id]) > 0
    }

    /// Libera os componentes `ids` (e os descendentes) de uma vez: o mesmo que [`free`]
    /// em cada um, numa passada só pelo resto da árvore. Esvaziar uma lista de centenas de
    /// linhas custava uma passada por linha. Devolve quantos dos `ids` existiam.
    ///
    /// [`free`]: Self::free
    pub fn free_many(&mut self, ids: &[ComponentId]) -> usize {
        let mut doomed = Vec::new();
        let mut doomed_set = std::collections::HashSet::new();
        let mut tops = Vec::new();
        for &id in ids {
            if self.contains(id) && !doomed_set.contains(&id) {
                let start = doomed.len();
                self.collect_subtree(id, &mut doomed);
                doomed_set.extend(doomed[start..].iter().copied());
                tops.push(id);
            }
        }
        if tops.is_empty() {
            return 0;
        }
        let gone = |c: &ComponentId| doomed_set.contains(c);
        // Os forms que saem (o backend remoto descarta as instâncias e os `PostTo` deles).
        let freed_forms: Vec<ComponentId> = doomed.iter().copied().filter(|&d| self.class_name(d) == Some("Form")).collect();
        // Quem fica e aponta para algum que sai (o `data_source` de um DBEdit para o
        // DataSource liberado): a referência é limpa depois, sozinha.
        let orphaned: Vec<(ComponentId, &'static str)> = self.referrers_of_all(&doomed).into_iter().filter(|(c, _)| !gone(c)).collect();
        // O índice de nomes só tem quem tem nome: liberar só controles sem nome (as linhas de
        // uma lista) não o derruba.
        let named = doomed.iter().any(|&d| self.name(d).is_some_and(|n| !n.is_empty()));
        // Os nomes que saem de cada escopo (lidos antes de soltar a subárvore).
        let leaving: Vec<(Option<ComponentId>, Vec<(ComponentId, String)>)> = if named { tops.iter().map(|&t| (self.name_scope_of(t), self.names_with(t))).collect() } else { Vec::new() };
        for &id in &tops {
            self.detach(id);
        }
        for &d in &doomed {
            let slot = &mut self.slots[d.index as usize];
            // A camada guardada dele sai do renderizador na próxima pintura.
            if let Some(key) = slot.entry.as_ref().and_then(|e| e.visual.borrow_mut().layer_key.take()) {
                self.dead_layers.get_mut().push(key);
            }
            slot.entry = None;
            slot.generation = slot.generation.wrapping_add(1);
            self.free_slots.push(d.index);
        }
        self.pending.retain(|(sender, _, _)| !gone(sender));
        self.color_names.retain(|(c, _), _| !gone(c));
        self.drop_tasks_of(&doomed);
        self.forget_instances(&doomed);
        self.forget_questions(&doomed);
        self.animations.retain(|a| !gone(&a.target) && a.owner.is_none_or(|o| !gone(&o)));
        self.input.retain(|root, _| !gone(root));
        self.forget_forms(&doomed);
        self.appear_waiting = None;
        for t in self.timers.extract_if(.., |t| gone(&t.id)).collect::<Vec<_>>() {
            t.stop();
        }
        for state in self.input.values_mut() {
            state.forget(&doomed);
        }
        for slot in &mut self.slots {
            if let Some(c) = slot.entry.as_mut().and_then(|e| e.component.as_mut()) {
                for &d in &doomed {
                    c.notification(d);
                }
                if let Some(cb) = c.control_mut() {
                    if cb.popup_menu.is_some_and(|m| gone(&m)) {
                        cb.popup_menu = None;
                    }
                    cb.forget_drag_image(&doomed);
                }
            }
        }
        // O índice de referências perde só o que saiu (remontá-lo a cada liberação percorria
        // todas as propriedades do app); o de nomes cai só se saiu alguém com nome.
        self.prune_references(&doomed_set);
        for (scope, items) in leaving {
            self.names_removed(scope, &items);
        }
        self.forget_scopes(&doomed);
        for (c, prop) in orphaned {
            if matches!(self.get_prop(c, prop), Ok(PropValue::Component(Some(t))) if gone(&t) || !self.contains(t)) {
                let _ = self.set_prop(c, prop, PropValue::Component(None));
            }
        }
        // Quem só vive com um dos liberados (a lista aberta de um ComboBox) vai junto.
        let dependents: Vec<ComponentId> = self
            .components()
            .filter(|&c| self.component(c).and_then(|c| c.dies_with()).is_some_and(|o| gone(&o)))
            .collect();
        self.free_many(&dependents);
        if !freed_forms.is_empty() {
            if let Some(hook) = self.busy.freed_hook.clone() {
                hook(self, &freed_forms);
            }
        }
        self.invalidate();
        tops.len()
    }

    fn collect_subtree(&self, id: ComponentId, out: &mut Vec<ComponentId>) {
        out.push(id);
        if let Some(e) = self.entry(id) {
            for &c in &e.children {
                self.collect_subtree(c, out);
            }
        }
    }

    /// Tira o componente da lista de filhos do pai (ou das raízes).
    fn detach(&mut self, id: ComponentId) {
        match self.entry(id).and_then(|e| e.parent) {
            Some(p) => {
                if let Some(e) = self.entry_mut(p) {
                    e.children.retain(|c| *c != id);
                }
            }
            None => self.roots.retain(|r| *r != id),
        }
    }

    // =====================================================================
    // Árvore
    // =====================================================================

    pub fn contains(&self, id: ComponentId) -> bool {
        self.entry(id).is_some()
    }

    pub fn parent(&self, id: ComponentId) -> Option<ComponentId> {
        self.entry(id)?.parent
    }

    /// Filhos na ordem (é também a ordem de alinhamento e de pintura).
    pub fn children(&self, id: ComponentId) -> &[ComponentId] {
        self.entry(id).map_or(&[], |e| e.children.as_slice())
    }

    pub fn roots(&self) -> &[ComponentId] {
        &self.roots
    }

    /// Raiz (form) à qual o componente pertence.
    pub fn root_of(&self, id: ComponentId) -> ComponentId {
        let mut current = id;
        while let Some(p) = self.parent(current) {
            current = p;
        }
        current
    }

    /// Troca o pai (como `Parent := X`). `None` transforma em raiz.
    /// Em que nível da árvore `id` está (uma raiz: 1).
    fn depth_of(&self, id: ComponentId) -> usize {
        let mut depth = 1;
        let mut current = self.parent(id);
        while let Some(c) = current {
            depth += 1;
            current = self.parent(c);
        }
        depth
    }

    /// Quantos níveis a subárvore de `id` tem (ela sozinha: 1). Sem recursão.
    fn height_of(&self, id: ComponentId) -> usize {
        let mut height = 0;
        let mut stack = vec![(id, 1)];
        while let Some((node, level)) = stack.pop() {
            height = height.max(level);
            stack.extend(self.children(node).iter().map(|&c| (c, level + 1)));
        }
        height
    }

    pub fn set_parent(&mut self, id: ComponentId, parent: Option<ComponentId>) -> Result<()> {
        if !self.contains(id) {
            return Err(Error::InvalidId);
        }
        let old_root = self.root_of(id);
        let old_scope = self.name_scope_of(id);
        let was_scope = self.is_name_scope(id);
        let moving = self.names_with(id);
        if let Some(p) = parent {
            if !self.contains(p) {
                return Err(Error::InvalidId);
            }
            let mut current = Some(p);
            while let Some(c) = current {
                if c == id {
                    return Err(Error::Cycle);
                }
                current = self.parent(c);
            }
            if self.depth_of(p) + self.height_of(id) > self.limits.0 {
                return Err(too_deep(self.limits.0));
            }
            // Os nomes que mudam de escopo precisam continuar únicos no novo. Um form
            // (escopo) leva os nomes dele junto; só o nome dele mesmo muda de escopo.
            let new_scope = self.form_of(p);
            if self.name_scope_of(id) != Some(new_scope) {
                let mut moving = Vec::new();
                self.collect_scope(id, &mut moving);
                for &n in &moving {
                    let name = &self.entry(n).expect("subárvore válida").name;
                    if self.find_in(new_scope, name).is_some_and(|other| other != n) {
                        return Err(Error::DuplicateName(name.clone()));
                    }
                }
            }
        }
        self.detach(id);
        let entry = self.entry_mut(id).expect("verificado acima");
        entry.parent = parent;
        // Uma raiz continua sendo um escopo ao ser embutida (`Form2.Parent := Panel1`).
        entry.scope |= parent.is_none();
        match parent {
            Some(p) => self.entry_mut(p).expect("verificado acima").children.push(id),
            None => self.roots.push(id),
        }
        if let Some(cb) = self.cb_mut(id) {
            cb.anchor_ref = None;
        }
        // Os dois forms: o que perdeu o controle e o que ganhou.
        self.invalidate_form(old_root);
        self.invalidate_form(id);
        // Os nomes da subárvore saem de um escopo e entram no outro (os índices são
        // atualizados, não remontados: o frame repetido entra sem custo na tela toda).
        let new_scope = self.name_scope_of(id);
        if was_scope || parent.is_some() {
            if old_scope != new_scope {
                self.names_removed(old_scope, &moving);
                self.names_added(new_scope, &moving);
            }
        } else {
            // Virou raiz sem ser escopo: os nomes dele passam a ser um escopo novo.
            self.forget_scope(old_scope);
            self.forget_scopes(&[id]);
        }
        Ok(())
    }

    /// Muda a posição do componente entre os irmãos (ordem de alinhamento e de pintura).
    pub fn set_child_index(&mut self, id: ComponentId, index: usize) -> Result<()> {
        let parent = self.entry(id).ok_or(Error::InvalidId)?.parent;
        let list = match parent {
            Some(p) => &mut self.entry_mut(p).expect("pai válido").children,
            None => &mut self.roots,
        };
        list.retain(|c| *c != id);
        list.insert(index.min(list.len()), id);
        self.invalidate_form(id);
        // Num escopo os nomes são únicos: a ordem não muda o índice dele, só o global.
        self.names_removed(None, &[]);
        Ok(())
    }

    // =====================================================================
    // Nomes
    // =====================================================================

    pub fn name(&self, id: ComponentId) -> Option<&str> {
        self.entry(id).map(|e| e.name.as_str())
    }

    /// Define o nome. Precisa ser um identificador (`letra|_` seguido de letras, dígitos
    /// ou `_`) e único dentro do form a que o componente pertence, sem diferenciar
    /// maiúsculas. Forms diferentes podem ter componentes com o mesmo nome (dois
    /// `btnSalvar`), como no Delphi. Os forms raiz têm nomes únicos entre si. Vazio = sem
    /// nome.
    pub fn set_name(&mut self, id: ComponentId, name: &str) -> Result<()> {
        if !self.contains(id) {
            return Err(Error::InvalidId);
        }
        if !name.is_empty() {
            if !is_identifier(name) {
                return Err(Error::InvalidName(name.to_string()));
            }
            // Entre as raízes o nome pode repetir: é o mesmo form aberto duas vezes (duas
            // janelas de cliente), cada uma com a sua instância. Dentro de um form, não.
            let other = self.name_scope_of(id).and_then(|scope| self.find_in(scope, name));
            if other.is_some_and(|o| o != id) {
                return Err(Error::DuplicateName(name.to_string()));
            }
        }
        let old = std::mem::replace(&mut self.entry_mut(id).expect("verificado acima").name, name.to_string());
        let scope = self.name_scope_of(id);
        self.names_removed(scope, &[(id, old)]);
        self.names_added(scope, &[(id, name.to_string())]);
        Ok(())
    }

    /// Um nome livre para um componente que vai entrar em `parent`: `base` se ninguém do
    /// form de `parent` o usa, senão `base2`, `base3`... (o frame repetido: cada cartão
    /// de uma lista com o seu, como no Delphi).
    pub fn free_name_in(&self, parent: ComponentId, base: &str) -> String {
        let scope = self.form_of(parent);
        if self.find_in(scope, base).is_none() {
            return base.to_string();
        }
        // Continua de onde a última parou: N instâncias custam N buscas, não N²/2.
        let first = self.next_suffix(scope, base);
        let n = (first..).find(|n| self.find_in(scope, &format!("{base}{n}")).is_none()).expect("sempre há um número livre");
        self.set_next_suffix(scope, base, n + 1);
        format!("{base}{n}")
    }

    /// Procura pelo nome em todos os forms, inclusive os embutidos (o primeiro
    /// encontrado). Com vários forms abertos, prefira [`find_in`](Self::find_in) ou
    /// [`find_path`](Self::find_path): o mesmo nome pode existir em mais de um.
    pub fn find(&self, name: &str) -> Option<ComponentId> {
        if name.is_empty() {
            return None;
        }
        self.indexed_find(name)
    }

    pub(crate) fn find_deep(&self, id: ComponentId, name: &str) -> Option<ComponentId> {
        let e = self.entry(id)?;
        if e.name.eq_ignore_ascii_case(name) {
            return Some(id);
        }
        e.children.iter().find_map(|&c| self.find_deep(c, name))
    }

    /// Procura pelo nome entre os componentes do form `scope` (como `FindComponent`):
    /// não entra nos forms embutidos nele, que têm os próprios nomes (mas encontra o
    /// form embutido pelo nome dele).
    pub fn find_in(&self, scope: ComponentId, name: &str) -> Option<ComponentId> {
        let e = self.entry(scope)?;
        if !name.is_empty() && e.name.eq_ignore_ascii_case(name) {
            return Some(scope);
        }
        self.find_below(scope, name)
    }

    /// Abaixo de `id` (sem contar ele): pelo índice quando `id` é um form (o caso comum,
    /// `frmX.comp`); senão, percorrendo os filhos dele.
    fn find_below(&self, id: ComponentId, name: &str) -> Option<ComponentId> {
        if name.is_empty() {
            return None;
        }
        if self.is_name_scope(id) { self.indexed_find_in_scope(id, name) } else { self.find_in_children(id, name) }
    }

    pub(crate) fn find_in_children(&self, id: ComponentId, name: &str) -> Option<ComponentId> {
        self.children(id).iter().find_map(|&c| {
            let e = self.entry(c)?;
            if !name.is_empty() && e.name.eq_ignore_ascii_case(name) {
                Some(c)
            } else if e.scope {
                None
            } else {
                self.find_in_children(c, name)
            }
        })
    }

    /// Procura por um caminho com pontos: `frmClientes.btnSalvar` (o form e o componente
    /// dentro dele), `frmPrincipal.frmClientes.btnSalvar` (form embutido). Sem ponto, é
    /// o mesmo que [`find`](Self::find).
    pub fn find_path(&self, path: &str) -> Option<ComponentId> {
        let mut parts = path.split('.');
        let first = parts.next()?;
        let mut current = self
            .roots
            .iter()
            .copied()
            .find(|&r| self.entry(r).is_some_and(|e| e.name.eq_ignore_ascii_case(first)))
            .or_else(|| self.find(first))?;
        for part in parts {
            current = self.find_below(current, part)?;
        }
        Some(current)
    }

    /// Caminho que [`find_path`](Self::find_path) entende: `frmClientes.btnSalvar`.
    pub fn path_of(&self, id: ComponentId) -> String {
        let mut parts = vec![self.name(id).unwrap_or_default().to_string()];
        let mut scope = self.name_scope_of(id);
        while let Some(s) = scope {
            parts.push(self.name(s).unwrap_or_default().to_string());
            scope = self.name_scope_of(s);
        }
        parts.reverse();
        parts.join(".")
    }

    /// Resolve uma referência escrita num componente (`data_source: "dsClientes"`): procura
    /// no form dele, depois nos forms em volta (um form embutido enxerga os componentes do
    /// form onde está). Com ponto, é um caminho: `"dmDados.dsClientes"` (outro form, como
    /// um DataModule do Delphi).
    pub fn resolve_reference(&self, from: ComponentId, text: &str) -> Option<ComponentId> {
        if text.contains('.') {
            return self.find_path(text);
        }
        let mut scope = Some(self.form_of(from));
        while let Some(s) = scope {
            if let Some(found) = self.find_in(s, text) {
                return Some(found);
            }
            scope = self.name_scope_of(s);
        }
        None
    }

    /// Como `from` deve escrever uma referência a `target` para que
    /// [`resolve_reference`](Self::resolve_reference) a encontre: o nome, ou o caminho
    /// completo se o alvo está em outro form. `None` se o alvo não tem nome.
    pub fn reference_name(&self, from: ComponentId, target: ComponentId) -> Option<String> {
        let name = self.name(target).filter(|n| !n.is_empty())?;
        if self.resolve_reference(from, name) == Some(target) {
            Some(name.to_string())
        } else {
            Some(self.path_of(target))
        }
    }

    /// O form a que o componente pertence: o escopo de nomes mais próximo, contando ele
    /// mesmo (para um form, ele próprio). Como o `Owner` de um componente no Delphi.
    pub fn form_of(&self, id: ComponentId) -> ComponentId {
        let mut current = id;
        loop {
            match self.entry(current) {
                Some(e) if !e.scope => match e.parent {
                    Some(p) => current = p,
                    None => return current,
                },
                _ => return current,
            }
        }
    }

    /// Escopo onde vive o nome de `id`: o form dono do pai (`None` para um form raiz).
    pub fn name_scope_of(&self, id: ComponentId) -> Option<ComponentId> {
        self.parent(id).map(|p| self.form_of(p))
    }

    /// Faz de `id` um escopo de nomes (o carregador de layouts faz isso com o form
    /// carregado dentro de outro). Os forms raiz já são.
    pub fn set_name_scope(&mut self, id: ComponentId, scope: bool) {
        if let Some(e) = self.entry_mut(id) {
            e.scope = scope || e.parent.is_none();
        }
        self.forget_scope(self.name_scope_of(id));
        self.forget_scopes(&[id]);
    }

    pub fn is_name_scope(&self, id: ComponentId) -> bool {
        self.entry(id).is_some_and(|e| e.scope)
    }

    /// Os componentes cujo nome está no mesmo escopo que o de `id` (ele e os descendentes
    /// que não estão dentro de outro form).
    pub(crate) fn collect_scope(&self, id: ComponentId, out: &mut Vec<ComponentId>) {
        out.push(id);
        if self.entry(id).is_some_and(|e| e.scope) {
            return;
        }
        for &c in self.children(id) {
            self.collect_scope(c, out);
        }
    }

    // =====================================================================
    // Acesso aos componentes
    // =====================================================================

    pub fn class_name(&self, id: ComponentId) -> Option<&'static str> {
        Some(self.component(id)?.class_name())
    }

    pub fn component(&self, id: ComponentId) -> Option<&dyn Component> {
        self.entry(id)?.component.as_deref()
    }

    pub fn component_mut(&mut self, id: ComponentId) -> Option<&mut dyn Component> {
        // Mexer no componente por fora pode mudar o que ele mostra nos filhos: a janela toda.
        self.invalidate_form(id);
        self.request_repaint();
        self.forget_references();
        self.entry_mut(id)?.component.as_deref_mut()
    }

    /// Acesso tipado. Procura também nas camadas de composição: num `GlowButton` que
    /// contém um `Button`, `get::<Button>` devolve o `Button` interno (como um cast
    /// `TButton(Sender)` no Delphi).
    pub fn get<T: Component>(&self, id: ComponentId) -> Option<&T> {
        find_layer(self.component(id)?)
    }

    pub fn get_mut<T: Component>(&mut self, id: ComponentId) -> Option<&mut T> {
        self.invalidate_form(id);
        self.request_repaint();
        self.forget_references();
        find_layer_mut(self.entry_mut(id)?.component.as_deref_mut()?)
    }

    /// Dados de controle visual (`None` para componentes não visuais).
    pub fn control(&self, id: ComponentId) -> Option<&ControlBase> {
        self.cb(id)
    }

    pub fn control_mut(&mut self, id: ComponentId) -> Option<&mut ControlBase> {
        self.invalidate_form(id);
        self.cb_mut(id)
    }

    /// Posição e tamanho reais, relativos ao pai, sempre em dia: com o layout pendente (algo
    /// mudou de tamanho, apareceu, trocou de aba), ele roda antes (como o realinhamento
    /// imediato da VCL). Sem nada pendente, custa só conferir. Para ler sem refazer o layout
    /// (num `&Ui`, na pintura), [`laid_out_bounds`](Self::laid_out_bounds).
    pub fn bounds(&mut self, id: ComponentId) -> Option<RectF> {
        self.update_layout();
        self.laid_out_bounds(id)
    }

    /// Posição e tamanho do último layout, sem refazê-lo: o que já está desenhado.
    pub fn laid_out_bounds(&self, id: ComponentId) -> Option<RectF> {
        Some(self.cb(id)?.bounds)
    }

    /// Posição real relativa ao form de um retângulo dado em coordenadas do **pai** de
    /// `id`. Útil dentro dos métodos de evento do próprio componente (quando ele está
    /// fora do `Ui` e `absolute_rect(ctx.id)` não o encontra): passe `self.base.bounds()`.
    /// Área segura do form, informada pela plataforma do celular: as faixas cobertas pela
    /// barra de status, pela barra de navegação, pelo recorte da câmera ou pelo teclado
    /// virtual. O fundo do form continua na tela toda; os filhos ficam fora dessas faixas
    /// (somadas ao `padding`).
    pub fn set_safe_area(&mut self, root: ComponentId, insets: Bounds) {
        let Some(cb) = self.cb_mut(root) else { return };
        if cb.safe_area != insets {
            cb.safe_area = insets;
            self.invalidate_form(root);
        }
    }

    pub fn safe_area(&self, root: ComponentId) -> Bounds {
        self.cb(root).map_or_else(Bounds::default, |cb| cb.safe_area)
    }

    pub fn to_form(&self, id: ComponentId, rect_in_parent: RectF) -> RectF {
        let origin = self.parent(id).and_then(|p| self.absolute_rect(p)).map_or(crate::geometry::PointF::ZERO, |r| r.origin());
        rect_in_parent.offset(origin.x, origin.y)
    }

    /// As vistas nativas do form (WebView...), com a posição na janela (descontada a
    /// rolagem dos contêineres) e se aparecem. No modo de design não há nenhuma: o
    /// designer mostra o desenho do controle. Os comandos pendentes são entregues aqui.
    pub fn native_views(&self, form: ComponentId) -> Vec<crate::NativeViewSlot> {
        let mut out = Vec::new();
        if self.design_mode() {
            return out;
        }
        let mut stack = vec![(form, true)];
        while let Some((id, shown)) = stack.pop() {
            let shown = shown && self.control(id).is_none_or(|c| c.visible());
            if let Some(view) = self.component(id).and_then(|c| c.platform_hooks()).and_then(|h| h.native_view()) {
                // A rolagem já está nos `bounds` dos filhos (o layout desloca cada um pelo
                // `scroll_offset` do pai): descontá-la de novo punha a vista nativa numa caixa
                // rolada o dobro mais longe.
                let rect = self.absolute_rect(id).unwrap_or_default();
                // O recorte dos pais que recortam os filhos (a caixa rolada): a vista nativa fica
                // por cima de tudo e, sem ele, passaria por cima do que está em volta.
                let mut clip: Option<RectF> = None;
                let mut parent = self.parent(id);
                while let Some(p) = parent {
                    if self.control(p).is_some_and(|cb| cb.clip_children()) {
                        let r = self.absolute_rect(p).unwrap_or_default();
                        clip = Some(clip.map_or(r, |c| c.intersect(r).unwrap_or(RectF::new(r.x, r.y, 0.0, 0.0))));
                    }
                    parent = self.parent(p);
                }
                let clip = clip.filter(|c| !(c.x <= rect.x && c.y <= rect.y && c.right() >= rect.right() && c.bottom() >= rect.bottom()));
                let visible = shown && rect.width > 0.0 && rect.height > 0.0 && clip.is_none_or(|c| c.intersect(rect).is_some_and(|i| i.width > 0.0 && i.height > 0.0));
                let commands = self.component(id).and_then(|c| c.platform_hooks()).map(|h| h.take_native_commands()).unwrap_or_default();
                out.push(crate::NativeViewSlot { id, rect, clip, visible, view, commands });
            }
            for c in self.children(id) {
                stack.push((*c, shown));
            }
        }
        out
    }

    /// Posição e tamanho reais relativos ao form.
    pub fn absolute_rect(&self, id: ComponentId) -> Option<RectF> {
        let mut rect = self.laid_out_bounds(id)?;
        let mut parent = self.parent(id);
        while let Some(p) = parent {
            if let Some(b) = self.laid_out_bounds(p) {
                rect = rect.offset(b.x, b.y);
            }
            parent = self.parent(p);
        }
        Some(rect)
    }

    // =====================================================================
    // Registro de classes, tema, serviços
    // =====================================================================

    /// Registra uma classe criável por nome. Usa o `Default` do tipo.
    pub fn register_class<T: Component + Default>(&mut self, category: &'static str) {
        let name = T::default().class_name();
        self.registry.register(ClassInfo { name, category, create: create_default::<T> });
    }

    pub fn registry(&self) -> &ClassRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut ClassRegistry {
        &mut self.registry
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn theme_mut(&mut self) -> &mut Theme {
        self.request_repaint();
        self.recolored.borrow_mut().clear();
        std::sync::Arc::make_mut(&mut self.theme)
    }

    /// Troca o tema (todos os controles são repintados).
    pub fn set_theme(&mut self, theme: Theme) {
        self.set_shared_theme(std::sync::Arc::new(theme));
    }

    /// O mesmo que [`Ui::set_theme`], sem copiar o tema: vários `Ui` com o mesmo tema (a IDE
    /// abre um por aba de form) guardam um só. Com a paleta do projeto, este `Ui` fica com uma
    /// cópia já com as cores dela.
    pub fn set_shared_theme(&mut self, mut theme: std::sync::Arc<Theme>) {
        if !self.palette.is_empty() {
            std::sync::Arc::make_mut(&mut theme).apply_palette(&self.palette);
        }
        self.theme = theme;
        self.recolored.borrow_mut().clear();
        self.refresh_color_names();
        self.request_repaint();
    }

    /// A paleta do projeto (as cores com nome do `.aylisproj`): entra no tema em uso, nos da
    /// biblioteca e em todo tema posto depois com [`Ui::set_theme`].
    pub fn set_palette(&mut self, palette: crate::palette::Palette) {
        self.palette = palette;
        // Sem cores na paleta, nada muda: os temas continuam compartilhados.
        if !self.palette.is_empty() {
            std::sync::Arc::make_mut(&mut self.theme).apply_palette(&self.palette);
            for t in &mut self.themes {
                std::sync::Arc::make_mut(t).apply_palette(&self.palette);
            }
        }
        self.recolored.borrow_mut().clear();
        self.refresh_color_names();
        self.request_repaint();
    }

    pub fn palette(&self) -> &crate::palette::Palette {
        &self.palette
    }

    /// Como a roda rola as caixas de rolagem do app: pontos por dente e se desliza. Cada
    /// `ScrollBox` pode trocar (`wheel_step`, `smooth`).
    pub fn scroll_options(&self) -> crate::ScrollOptions {
        self.scroll_options
    }

    pub fn set_scroll_options(&mut self, options: crate::ScrollOptions) {
        self.scroll_options = options;
    }

    /// Os nomes de cor que existem agora (os do tema em uso, com os da paleta), em ordem.
    pub fn color_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.theme.colors.keys().cloned().collect();
        names.sort();
        names
    }

    /// O nome com que a propriedade de cor foi escrita (`None` = uma cor fixa).
    pub fn color_name(&self, id: ComponentId, prop: &str) -> Option<&str> {
        let (name, given) = self.color_names.get(&(id, prop.to_string()))?;
        // Trocada direto no campo (`get_mut`): a cor de agora é fixa, e o nome não vale mais.
        let now = self.component(id).and_then(|c| crate::component::read_prop(c, prop));
        match now {
            Some(v) if v != PropValue::Color(*given) => None,
            _ => Some(name.as_str()),
        }
    }

    /// Recalcula as propriedades de cor escritas com nome (depois de mudar o tema).
    pub fn refresh_color_names(&mut self) {
        let names: Vec<(ComponentId, String)> = self.color_names.keys().cloned().collect();
        for (id, prop) in names {
            // Liberado, ou trocado direto no campo: o nome sai, e a cor de agora fica.
            let Some(spec) = self.color_name(id, &prop).filter(|_| self.contains(id)).map(str::to_string) else {
                self.color_names.remove(&(id, prop));
                continue;
            };
            let _ = self.set_prop_quiet(id, &prop, PropValue::String(spec));
        }
    }

    /// Acrescenta (ou substitui, pelo nome) um tema na biblioteca. Os temas da biblioteca
    /// podem ser escolhidos por controle (propriedade `theme`) ou por classe ([`Theme::mix`]).
    /// "Aylis Light" e "Aylis Dark" já vêm.
    pub fn register_theme(&mut self, mut theme: Theme) {
        theme.apply_palette(&self.palette);
        self.recolored.borrow_mut().clear();
        match self.themes.iter_mut().find(|t| t.name.eq_ignore_ascii_case(&theme.name)) {
            Some(t) => *t = std::sync::Arc::new(theme),
            None => self.themes.push(std::sync::Arc::new(theme)),
        }
        self.request_repaint();
    }

    /// A cor de fundo com que o controle é pintado (o `fill` do estilo dele no tema em uso;
    /// transparente se não tiver). A plataforma usa a do form principal para pintar as
    /// barras do sistema no celular.
    pub fn background_color(&self, id: ComponentId) -> crate::Color {
        let Some(component) = self.component(id) else { return crate::Color::TRANSPARENT };
        let Some(cb) = component.control() else { return crate::Color::TRANSPARENT };
        let mut classes = Vec::new();
        component.class_chain(&mut classes);
        let base = match cb.theme_name() {
            "" => self.theme(),
            name => self.theme_named(name).unwrap_or(self.theme()),
        };
        let theme = self.mixed_theme(base, &classes);
        theme.resolve(&classes, cb.style(), cb.state()).fill
    }

    /// Um tema pelo nome: o atual, se o nome for o dele, um da biblioteca do app ou um dos
    /// temas de plataforma que o aylis traz ([`crate::themes`]). Só a família ("iOS 26")
    /// dá a versão clara ou escura conforme o tema do app.
    pub fn theme_named(&self, name: &str) -> Option<&Theme> {
        if self.theme.name.eq_ignore_ascii_case(name) {
            return Some(&self.theme);
        }
        self.themes
            .iter()
            .find(|t| t.name.eq_ignore_ascii_case(name))
            .map(|t| &**t)
            .or_else(|| crate::themes::builtin(name))
            // Só a família ("Windows 11"): a versão clara ou escura, como o tema do app.
            .or_else(|| crate::themes::of_family(name, self.theme.dark))
    }

    /// Os nomes dos temas disponíveis: o atual, os do app e os de plataforma do aylis.
    pub fn theme_names(&self) -> Vec<String> {
        let mut names: Vec<String> = vec![self.theme.name.clone()];
        let others = self.themes.iter().map(|t| t.name.clone()).chain(crate::themes::all().iter().map(|t| t.name.clone()));
        for n in others {
            if !names.iter().any(|x| x.eq_ignore_ascii_case(&n)) {
                names.push(n);
            }
        }
        names
    }

    /// De onde vem o tema de um controle (para a IDE mostrar quem usa qual tema).
    pub fn theme_source(&self, id: ComponentId) -> (String, ThemeSource) {
        let mut cur = Some(id);
        let mut inherited: Option<(&Theme, ThemeSource)> = None;
        while let Some(c) = cur {
            if let Some(name) = self.control(c).map(|cb| cb.theme_name()).filter(|n| !n.is_empty()) {
                if let Some(t) = self.theme_named(name) {
                    let source = if c == id { ThemeSource::Own } else { ThemeSource::Inherited(c) };
                    inherited = Some((t, source));
                    break;
                }
            }
            cur = self.parent(c);
        }
        let (base, source) = inherited.unwrap_or((&self.theme, ThemeSource::App));
        let mut classes = Vec::new();
        if let Some(component) = self.entry(id).and_then(|e| e.component.as_deref()) {
            component.class_chain(&mut classes);
        }
        if let Some(class) = classes.iter().find(|c| base.components.contains_key(**c)) {
            let mixed = self.mixed_theme(base, &classes);
            if !std::ptr::eq(mixed, base) {
                return (mixed.name.clone(), ThemeSource::Class(class.to_string()));
            }
        }
        (base.name.clone(), source)
    }

    /// O tema com que o controle é pintado: o da propriedade `theme` dele ou do ancestral
    /// mais próximo que tiver uma, e depois a mistura por classe ([`Theme::mix`]).
    pub fn theme_of(&self, id: ComponentId) -> &Theme {
        let mut inherited: &Theme = &self.theme;
        let mut cur = Some(id);
        while let Some(c) = cur {
            if let Some(name) = self.control(c).map(|cb| cb.theme_name()).filter(|n| !n.is_empty()) {
                if let Some(t) = self.theme_named(name) {
                    inherited = t;
                    break;
                }
            }
            cur = self.parent(c);
        }
        let mut classes = Vec::new();
        if let Some(component) = self.entry(id).and_then(|e| e.component.as_deref()) {
            component.class_chain(&mut classes);
        }
        self.mixed_theme(inherited, &classes)
    }

    /// Aplica a mistura por classe de `theme`.
    pub(crate) fn mixed_theme<'a>(&'a self, theme: &'a Theme, classes: &[&str]) -> &'a Theme {
        theme.mixed_for(classes).and_then(|name| self.theme_named(name)).unwrap_or(theme)
    }

    /// Registra um ícone do app (substitui um do aylis com o mesmo nome).
    pub fn register_icon(&mut self, name: &str, icon: crate::icons::Icon) {
        self.icons.insert(name.to_string(), icon);
        self.request_repaint();
    }

    /// Registra um ícone a partir de caminhos SVG (`d`), na grade `size` × `size`.
    pub fn register_icon_svg(&mut self, name: &str, stroke: &str, fill: &str, size: f32) -> crate::Result<()> {
        let icon = crate::icons::Icon::from_svg(stroke, fill, size)
            .ok_or_else(|| crate::Error::invalid(format!("caminho SVG inválido no ícone '{name}'")))?;
        self.register_icon(name, icon);
        Ok(())
    }

    /// Registra todos os ícones de um pacote (`aylis-icons`) com o prefixo
    /// (`"material"` → `"material:home"`). Devolve quantos.
    pub fn register_icon_pack(&mut self, prefix: &str, text: &str) -> crate::Result<usize> {
        let pack = crate::icons::parse_pack(text).map_err(crate::Error::invalid)?;
        let mut n = 0;
        for (name, d) in &pack.icons {
            if let Some(icon) = pack.build(d) {
                self.icons.insert(format!("{prefix}:{name}"), icon);
                n += 1;
            }
        }
        self.request_repaint();
        Ok(n)
    }

    /// Um ícone pelo nome: os do app primeiro, depois os do aylis.
    pub fn icon(&self, name: &str) -> Option<&crate::icons::Icon> {
        self.icons.get(name).or_else(|| crate::icons::builtin(name))
    }

    pub fn services(&self) -> &Services {
        &self.services
    }

    pub fn services_mut(&mut self) -> &mut Services {
        &mut self.services
    }

    pub fn set_services(&mut self, services: Services) {
        self.services = services;
    }

    /// Mede um texto numa linha só. Usa o [`TextService`](crate::TextService) da
    /// plataforma; sem ele (em testes), faz uma estimativa pelo tamanho da fonte.
    pub fn measure_text(&self, text: &str, style: &TextStyle) -> SizeF {
        if self.text_mode != crate::textcore::TextMode::Native {
            if let Some(size) = crate::textcore::measure(text, style, None) {
                return size;
            }
        }
        match &self.services.text {
            Some(service) => service.measure(text, style, None),
            None => SizeF::new(text.chars().count() as f32 * style.font.size * 0.55, style.font.size * 1.3),
        }
    }

    /// Mede um texto quebrado na largura `max_width` (a altura de um rótulo com
    /// `wrap`). Sem o serviço da plataforma, estima pelo tamanho da fonte.
    pub fn measure_text_wrapped(&self, text: &str, style: &TextStyle, max_width: f32) -> SizeF {
        if self.text_mode != crate::textcore::TextMode::Native {
            if let Some(size) = crate::textcore::measure(text, style, Some(max_width)) {
                return size;
            }
        }
        match &self.services.text {
            Some(service) => service.measure(text, style, Some(max_width)),
            None => {
                let one = self.measure_text(text, style);
                let lines = (one.width / max_width.max(1.0)).ceil().max(1.0);
                SizeF::new(one.width.min(max_width), one.height * lines)
            }
        }
    }

    /// Formatos regionais: os do serviço da plataforma ou, sem ele, o padrão
    /// ([`LocaleInfo::default`](crate::platform::LocaleInfo)).
    pub fn locale(&self) -> crate::platform::LocaleInfo {
        self.services.locale.as_ref().map(|l| l.locale()).unwrap_or_default()
    }

    /// Horário (em [`Ui::time`]) do próximo frame pedido por algum controle, por exemplo o
    /// cursor piscando. A plataforma dorme até lá.
    /// Só o horário dos quadros pedidos na pintura (`request_frame_at`), sem os timers: quando
    /// ele chega, a plataforma repinta; um timer vencido só roda (e pede repintura se mudar
    /// algo).
    pub fn next_paint_wake(&self) -> Option<f64> {
        self.frame_requests.borrow().iter().filter(|(id, _)| self.contains(**id)).filter_map(|(_, f)| f.1).reduce(f64::min)
    }

    pub fn next_wake(&self) -> Option<f64> {
        let painted = self.frame_requests.borrow().iter().filter(|(id, _)| self.contains(**id)).filter_map(|(_, f)| f.1).reduce(f64::min);
        match (painted, self.timers_wake()) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }

    /// O zoom do app inteiro: 1 é o normal, 1.5 deixa tudo 50% maior (textos, controles,
    /// imagens), como o zoom do navegador. O layout se refaz no espaço que sobra. Cada form
    /// ainda multiplica pelo `zoom` dele ([`form_zoom`](Self::form_zoom)).
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Muda o zoom do app (de 0.25 a 5).
    pub fn set_zoom(&mut self, zoom: f32) {
        let zoom = if zoom.is_finite() { zoom.clamp(0.25, 5.0) } else { 1.0 };
        if (zoom - self.zoom).abs() > f32::EPSILON {
            self.zoom = zoom;
            self.mark_zoom_changed();
        }
    }

    /// O zoom em que o form `form` aparece: o do app vezes o `zoom` do form.
    pub fn form_zoom(&self, form: ComponentId) -> f32 {
        let own = match self.get_prop(form, "zoom") {
            Ok(PropValue::Float(z)) if z > 0.0 => z as f32,
            Ok(PropValue::Int(z)) if z > 0 => z as f32,
            _ => 1.0,
        };
        (self.zoom * own).clamp(0.25, 5.0)
    }

    /// Avisa a plataforma que um zoom mudou (o `zoom` de um form chama isto).
    pub fn mark_zoom_changed(&mut self) {
        self.zoom_changed = true;
        self.request_repaint();
    }

    /// Se algum zoom mudou desde a última vez (a plataforma pergunta a cada volta).
    pub fn take_zoom_changed(&mut self) -> bool {
        std::mem::take(&mut self.zoom_changed)
    }

    /// Ctrl + `+`/`-`/`0` e Ctrl + roda mudam o zoom do app (desligado por padrão).
    pub fn zoom_keys(&self) -> bool {
        self.zoom_keys
    }

    pub fn set_zoom_keys(&mut self, on: bool) {
        self.zoom_keys = on;
    }

    /// As fontes do projeto por nome (`titulo` → `Montserrat`), como as cores do projeto: um
    /// componente com `font_family: "titulo"` usa a família desse nome, e trocar a família
    /// aqui troca em todos de uma vez (repinta).
    pub fn set_font_names(&mut self, names: Vec<(String, String)>) {
        self.font_names = names;
        self.request_repaint();
    }

    pub fn font_names(&self) -> &[(String, String)] {
        &self.font_names
    }

    /// A família de verdade de `family`: a do nome, se for uma fonte do projeto (até dois
    /// níveis: um nome pode apontar para outro); senão ela mesma.
    pub fn resolve_font(&self, family: &str) -> String {
        let mut current = family;
        for _ in 0..3 {
            match self.font_names.iter().find(|(name, _)| name == current) {
                Some((_, target)) => current = target,
                None => break,
            }
        }
        current.to_string()
    }

    /// Tempo atual em segundos (informado pela plataforma; usado nas animações).
    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn set_time(&mut self, seconds: f64) {
        self.time = seconds;
    }

    /// O relógio falso dos testes: anda `seconds` e roda os relógios que venceram (os `Timer`,
    /// o `set_timeout`), sem plataforma (o relatório do Tabernáculo, P08).
    pub fn advance(&mut self, seconds: f64) {
        self.time += seconds.max(0.0);
        self.run_timers();
    }

    // =====================================================================
    // Propriedades por nome
    // =====================================================================

    /// Todas as propriedades publicadas do componente: as da classe, as das camadas de
    /// composição e as do [`ControlBase`]. Além delas, todo componente tem `name`.
    pub fn prop_infos(&self, id: ComponentId) -> Vec<PropInfo> {
        self.component(id).map(crate::component::prop_infos_of).unwrap_or_default()
    }

    pub fn prop_info(&self, id: ComponentId, name: &str) -> Option<PropInfo> {
        crate::component::prop_info_of(self.component(id)?, name)
    }

    /// Lê uma propriedade pelo nome (como `GetPropValue`).
    ///
    /// Aceita subpropriedades de `Bounds`: `"margins.left"`, `"padding.top"`...
    /// Eventos devolvem o nome do handler; um handler definido por closure devolve
    /// `Event(None)`, porque não tem nome.
    pub fn get_prop(&self, id: ComponentId, name: &str) -> Result<PropValue> {
        self.read_prop(id, name, true)
    }

    /// O valor declarado, sem o tamanho que o layout deu ao alinhado: o que se grava no form.
    pub fn get_declared_prop(&self, id: ComponentId, name: &str) -> Result<PropValue> {
        self.read_prop(id, name, false)
    }

    fn read_prop(&self, id: ComponentId, name: &str, laid_out: bool) -> Result<PropValue> {
        let entry = self.entry(id).ok_or(Error::InvalidId)?;
        if name == "name" {
            return Ok(PropValue::String(entry.name.clone()));
        }
        if let Some((head, field)) = name.split_once('.') {
            let bounds = self.get_prop(id, head)?.to_bounds().map_err(|e| e.with_property(head))?;
            return bounds_field(&bounds, field)
                .map(PropValue::from)
                .ok_or_else(|| Error::UnknownProperty(name.to_string()));
        }
        // Fora do `Ui` (um método dele rodando), ele não responde: sem isto, a propriedade
        // pareceria não existir.
        if entry.component.is_none() {
            return Err(Error::Busy(self.path_of(id)));
        }
        // O tamanho de um controle alinhado é o que o layout deu (o `Width` do Delphi), com o app
        // rodando; no designer, o declarado (é o que se grava no form). O relatório do
        // Tabernáculo, L13: a largura de um controle encaixado vinha 100, a declarada.
        // Só o lado que o alinhamento estica (a largura do `Top`, a altura do `Left`) e só
        // visível: o escondido não passou pelo layout e guarda um tamanho velho.
        if laid_out && matches!(name, "width" | "height") && !self.design_mode
            && let Some(cb) = self.cb(id)
            && cb.visible()
            && stretches(cb.align(), name == "width")
        {
            let b = cb.bounds();
            let real = if name == "width" { b.width } else { b.height };
            if real > 0.0 {
                return Ok(PropValue::Float(real as f64));
            }
        }
        let info = self.prop_info(id, name).ok_or_else(|| Error::UnknownProperty(name.to_string()))?;
        if info.kind == PropKind::Color {
            if let Some(spec) = self.color_name(id, name) {
                return Ok(PropValue::String(spec.to_string()));
            }
        }
        if info.kind == PropKind::Event {
            let handler = entry.events.iter().find(|(n, _)| n == name).and_then(|(_, h)| match h {
                Handler::Named(n) => Some(n.clone()),
                Handler::Closure(_) => None,
            });
            return Ok(PropValue::Event(handler));
        }
        if self.design_mode && name == "visible" {
            if let Some(cb) = self.cb(id) {
                return Ok(PropValue::Bool(!cb.design_hidden));
            }
        }
        let component = entry.component.as_deref().ok_or_else(|| Error::Busy(self.path_of(id)))?;
        crate::component::read_prop(component, name).ok_or_else(|| Error::UnknownProperty(name.to_string()))
    }

    /// Grava uma propriedade pelo nome (como `SetPropValue`).
    ///
    /// - `"name"` muda o nome do componente;
    /// - `"margins.left"` e afins alteram um lado de um `Bounds`;
    /// - propriedades de referência aceitam o **nome** do componente, procurado no form;
    /// - eventos aceitam o nome de um handler (veja [`register_handler`](Self::register_handler)).
    pub fn set_prop(&mut self, id: ComponentId, name: &str, value: impl Into<PropValue>) -> Result<()> {
        let value = value.into();
        if !self.contains(id) {
            return Err(Error::InvalidId);
        }
        if name == "name" {
            let new_name = value.to_str().map_err(|e| e.with_property(name))?.to_string();
            return self.set_name(id, &new_name);
        }
        if let Some((head, field)) = name.split_once('.') {
            let mut bounds = self.get_prop(id, head)?.to_bounds().map_err(|e| e.with_property(head))?;
            let v = value.to_f32().map_err(|e| e.with_property(name))?;
            *bounds_field_mut(&mut bounds, field).ok_or_else(|| Error::UnknownProperty(name.to_string()))? = v;
            return self.set_prop(id, head, bounds);
        }
        if self.entry(id).is_some_and(|e| e.component.is_none()) {
            let error = Error::Busy(self.path_of(id));
            self.report_busy(&error);
            return Err(error);
        }

        let info = self.prop_info(id, name).ok_or_else(|| Error::UnknownProperty(name.to_string()))?;
        if info.read_only {
            return Err(Error::ReadOnly(name.to_string()));
        }
        // Cor com nome (`success`, `brand`, `text/85`): vira a cor do tema em uso e o nome
        // fica guardado. Qualquer outro valor desfaz o nome.
        let mut color_name = None;
        let value = match (&info.kind, value) {
            (PropKind::Color, PropValue::String(spec)) if crate::palette::is_named(&spec) => {
                let color = self
                    .theme
                    .resolve_color(&spec)
                    .ok_or_else(|| Error::invalid(format!("cor desconhecida '{spec}' (use #hex ou um nome: {})", self.color_names().join(", "))).with_property(name))?;
                color_name = Some((spec, color));
                PropValue::Color(color)
            }
            (_, v) => v,
        };
        if info.kind == PropKind::Color {
            match &color_name {
                Some(spec) => {
                    self.color_names.insert((id, name.to_string()), spec.clone());
                }
                None => {
                    self.color_names.remove(&(id, name.to_string()));
                }
            }
        }
        if info.kind == PropKind::Component {
            self.forget_references();
        }
        let value = match info.kind {
            PropKind::Event => {
                let handler = value.to_event().map_err(|e| e.with_property(name))?;
                // O nome vira função no código gerado: só identificador, nada que injete código.
                if let Some(h) = handler.as_deref().filter(|h| !is_identifier(h)) {
                    return Err(Error::invalid(format!("nome de handler inválido '{h}' (letras, dígitos e _, sem começar por dígito)")).with_property(name));
                }
                self.set_event_handler(id, name, handler.map(Handler::Named));
                return Ok(());
            }
            PropKind::Component => match &value {
                PropValue::String(target) | PropValue::Enum(target) => {
                    if target.is_empty() {
                        PropValue::Component(None)
                    } else {
                        let found = self
                            .resolve_reference(id, target)
                            .ok_or_else(|| Error::ComponentNotFound(target.clone()))?;
                        PropValue::Component(Some(found))
                    }
                }
                _ => value,
            },
            _ => value,
        };
        // No designer, `visible` desligado não tira o controle da tela: ele fica translúcido e
        // clicável (a visibilidade de verdade continua com o próprio controle: as páginas de
        // um TabControl, uma gaveta fechada...).
        let value = if self.design_mode && name == "visible" && self.cb(id).is_some() {
            let hidden = !value.to_bool().map_err(|e| e.with_property(name))?;
            let shown = !hidden || self.design_revealed(id);
            let cb = self.cb_mut(id).expect("verificado acima");
            cb.design_hidden = hidden;
            // Escondido e fora da seleção: some também no designer (sem ocupar lugar).
            cb.set_visible(shown);
            PropValue::Bool(cb.visible())
        } else {
            value
        };

        // Só a pintura muda (cores, opacidade, giro, o valor de um medidor): repinta sem refazer
        // o layout da tela (com centenas de controles, refazer a cada valor pesava).
        let paint_only = info.kind == PropKind::Color
            || PAINT_ONLY.contains(&name)
            || self.entry(id).and_then(|e| e.component.as_deref()).is_some_and(|c| c.paint_only_property(name));
        let before = self.cb(id).map(|cb| cb.state());
        // O valor do controle mudando pelo código: o `on_change` dispara depois (como na VCL).
        let old_value = if self.announces_change(id, &info) {
            self.entry(id).and_then(|e| e.component.as_deref()).and_then(|c| crate::component::read_prop(c, name))
        } else {
            None
        };
        let entry = self.entry_mut(id).expect("verificado acima");
        let component = entry.component.as_deref_mut().expect("ocupado já verificado acima");
        let result = match set_in_chain(component, name, value.clone()) {
            Err(Error::UnknownProperty(_)) => match component.control_mut() {
                Some(cb) => cb.set_prop(name, value),
                None => Err(Error::UnknownProperty(name.to_string())),
            },
            other => other,
        };
        if paint_only && matches!(name, "opacity" | "translate_x" | "translate_y") {
            // Só o lugar ou a transparência: o desenho de dentro é o mesmo (a camada guardada
            // dele continua valendo, veja `cache_layer`).
            self.damage_placement(id);
        } else if paint_only {
            self.damage(id);
        } else {
            self.invalidate_form(id);
        }
        if let Some(before) = before {
            self.sync_state_triggers(id, before);
        }
        if result.is_ok() {
            self.forget_translation_source(id, name);
            self.sync_timer(id);
            let _ = self.with_component(id, |c, ctx| c.property_changed(ctx, name));
            if name == "visible" {
                self.drop_hidden_focus(id);
            }
            if let Some(old) = old_value {
                self.fire_value_change(id, name, old);
            }
        }
        result.map_err(|e| e.with_property(name))
    }

    /// Grava sem disparar o `on_change` do valor: para o controle que muda o próprio valor
    /// por uma ação do usuário e dispara ele mesmo (com a gravação de volta no campo ligado).
    pub fn set_prop_quiet(&mut self, id: ComponentId, name: &str, value: impl Into<PropValue>) -> Result<()> {
        let value = value.into();
        self.without_change_events(|ui| ui.set_prop(id, name, value))
    }

    /// Roda `f` sem o `on_change` que o `set_prop` dispara quando o valor de um controle muda
    /// pelo código: carregar um form (como o `.dfm` na VCL), o hot reload, copiar um
    /// componente, aplicar as vistas por largura.
    pub fn without_change_events<R>(&mut self, f: impl FnOnce(&mut Ui) -> R) -> R {
        self.quiet_changes += 1;
        let result = f(self);
        self.quiet_changes -= 1;
        result
    }

    /// O `set_prop` agora dispara o `on_change` do valor? (Não no designer nem dentro de
    /// [`without_change_events`](Self::without_change_events).)
    pub fn change_events_enabled(&self) -> bool {
        !self.design_mode && self.quiet_changes == 0
    }

    /// Mudar `info` de `id` pelo código dispara o `on_change`: é o valor do controle, ele tem
    /// o evento e alguém escuta (um handler ou um `EventInterceptor`). Barato para o resto: o
    /// nome é conferido antes de procurar o evento.
    fn announces_change(&self, id: ComponentId, info: &PropInfo) -> bool {
        if !self.change_events_enabled() || !crate::component::is_value_property(info) {
            return false;
        }
        // A barra de progresso tem `value` e não tem o evento: para aqui, sem olhar os
        // interceptadores.
        if !self.prop_info(id, "on_change").is_some_and(|i| i.kind == PropKind::Event) {
            return false;
        }
        self.entry(id).is_some_and(|e| e.events.iter().any(|(n, _)| n == "on_change")) || self.is_intercepted(id, "on_change")
    }

    /// O valor de `name` mudou pelo código: dispara o `on_change` com os argumentos que o
    /// controle usa ([`Component::change_args`]). Sem gravar de volta no campo ligado (é o
    /// campo que pode estar chegando na tela, como no `TDBEdit`).
    fn fire_value_change(&mut self, id: ComponentId, name: &str, old: PropValue) {
        let Some(component) = self.entry(id).and_then(|e| e.component.as_deref()) else { return };
        let Some(new) = crate::component::read_prop(component, name) else { return };
        if new == old {
            return;
        }
        let Some(args) = component.change_args(name, &new) else { return };
        if self.intercept(id, "on_change", &args).is_none() {
            self.fire_handler(id, "on_change", &args);
        }
    }

    /// `id` sumiu: se o foco do teclado estava nele (ou dentro dele, como um campo de uma
    /// tela que foi escondida), sai (com o `on_exit`), para a digitação não ir para um
    /// controle que não está na tela.
    fn drop_hidden_focus(&mut self, id: ComponentId) {
        if self.cb(id).is_none_or(|cb| cb.visible()) {
            return;
        }
        let root = self.root_of(id);
        let Some(focused) = self.focused(root) else { return };
        let mut current = Some(focused);
        while let Some(c) = current {
            if c == id {
                self.clear_focus(root);
                return;
            }
            current = self.parent(c);
        }
    }

    /// Se o próprio componente mudou o estado (por exemplo um CheckBox marcando
    /// `checked`), dispara as animações correspondentes.
    fn sync_state_triggers(&mut self, id: ComponentId, before: ControlState) {
        use crate::anim::Trigger;
        let Some(after) = self.cb(id).map(|cb| cb.state()) else { return };
        for (trigger, was, is) in [
            (Trigger::Hover, before.hover, after.hover),
            (Trigger::Pressed, before.pressed, after.pressed),
            (Trigger::Focused, before.focused, after.focused),
            (Trigger::Checked, before.checked, after.checked),
        ] {
            if was != is {
                self.request_repaint();
                self.run_triggers(id, trigger, is);
            }
        }
    }

    // =====================================================================
    // Eventos
    // =====================================================================

    fn set_event_handler(&mut self, id: ComponentId, event: &str, handler: Option<Handler>) {
        if let Some(entry) = self.entry_mut(id) {
            entry.events.retain(|(n, _)| n != event);
            if let Some(h) = handler {
                entry.events.push((event.to_string(), h));
            }
        }
    }

    /// Há algum handler (closure ou por nome) ligado ao evento `event` do componente.
    pub fn has_handler(&self, id: ComponentId, event: &str) -> bool {
        self.entry(id).is_some_and(|e| e.events.iter().any(|(n, _)| n == event))
    }

    /// Liga uma closure a um evento (`"on_click"`, `"on_resize"`...).
    pub fn on(
        &mut self,
        id: ComponentId,
        event: &str,
        handler: impl Fn(&mut Ui, ComponentId, &EventArgs) + 'static,
    ) {
        // Um nome que a classe não tem (`"on_clik"`) nunca dispara: em modo dev, avisa (uma vez
        // por classe e evento). Não recusa: um componente pode disparar um evento que só
        // existe num modo dele (o `on_complete_request` do Memo editando código).
        if cfg!(debug_assertions) && self.contains(id) && self.prop_info(id, event).is_none_or(|p| p.kind != PropKind::Event) {
            let class = self.class_name(id).unwrap_or_default();
            thread_local!(static WARNED: RefCell<std::collections::HashSet<String>> = RefCell::default());
            if WARNED.with(|w| w.borrow_mut().insert(format!("{class}.{event}"))) {
                crate::stderr_line!("aylis: {class} não tem o evento {event}: o handler ligado com ui.on só roda se o componente disparar esse nome");
            }
        }
        self.set_event_handler(id, event, Some(Handler::Closure(std::rc::Rc::new(handler))));
    }

    /// Atalho para `on_click` (como `Button1.OnClick := ...`).
    pub fn on_click(&mut self, id: ComponentId, handler: impl Fn(&mut Ui, ComponentId) + 'static) {
        self.on(id, "on_click", move |ui, sender, _| handler(ui, sender));
    }

    /// Liga em `to` os mesmos handlers de `from` (closures e nomes): a cópia de um controle
    /// feita pelo próprio componente (a faixa de abas da janela nova) responde igual.
    pub fn copy_events(&mut self, from: ComponentId, to: ComponentId) {
        let events = self.entry(from).map(|e| e.events.clone()).unwrap_or_default();
        for (name, handler) in events {
            self.set_event_handler(to, &name, Some(handler));
        }
    }

    /// Desliga o handler de um evento.
    pub fn clear_event(&mut self, id: ComponentId, event: &str) {
        self.set_event_handler(id, event, None);
    }

    /// Registra um handler por nome. Eventos ligados a esse nome (por propriedade ou
    /// pelo arquivo de layout: `"on_click": "btn_salvar_click"`) chamam esta closure.
    ///
    /// Com vários forms, registre com o nome do form na frente
    /// (`"frmClientes.btn_salvar_click"`): esse vale só para os componentes daquele form
    /// e tem prioridade sobre o nome sem form.
    pub fn register_handler(
        &mut self,
        name: &str,
        handler: impl Fn(&mut Ui, ComponentId, &EventArgs) + 'static,
    ) {
        let (form, handler_name) = name.rsplit_once('.').unwrap_or(("", name));
        self.handlers.entry(form.to_string()).or_default().insert(handler_name.to_string(), std::rc::Rc::new(handler));
    }

    /// Define quem recebe eventos com nome de handler não registrado (C ABI, remoto).
    pub fn set_event_sink(
        &mut self,
        sink: impl Fn(&mut Ui, ComponentId, &str, &str, &EventArgs) + 'static,
    ) {
        self.event_sink = Some(std::rc::Rc::new(sink));
    }

    /// Pintura personalizada desta instância, mantendo todo o comportamento do controle.
    pub fn on_paint(
        &mut self,
        id: ComponentId,
        hook: impl Fn(&mut dyn Canvas, &PaintCtx<'_>, &dyn Fn(&mut dyn Canvas, &PaintCtx<'_>)) + 'static,
    ) {
        if let Some(entry) = self.entry_mut(id) {
            entry.paint_hook = Some(Box::new(hook));
        }
        self.request_repaint();
    }

    pub fn clear_on_paint(&mut self, id: ComponentId) {
        if let Some(entry) = self.entry_mut(id) {
            entry.paint_hook = None;
        }
        self.request_repaint();
    }

    /// Dispara agora o evento `event` do componente. Devolve `true` se algum handler
    /// tratou.
    ///
    /// Como no Delphi, um handler pode disparar outros eventos (inclusive o próprio, de outro
    /// componente ou do mesmo): eles são entregues na hora. Acima de [`MAX_EVENT_NESTING`]
    /// eventos aninhados, o evento não é entregue e vira um aviso (em vez de estourar a pilha).
    pub fn fire(&mut self, id: ComponentId, event: &str, args: &EventArgs) -> bool {
        // Um componente ligado a dados que o usuário mudou: grava de volta (antes do handler,
        // que já vê o campo novo).
        self.binding_changed(id, event);
        // Os `EventInterceptor` (item 18): antes do handler, num lugar só.
        if let Some(result) = self.intercept(id, event, args) {
            return result;
        }
        self.fire_handler(id, event, args)
    }

    /// O `fire` sem os interceptadores: só o handler ligado.
    pub(crate) fn fire_handler(&mut self, id: ComponentId, event: &str, args: &EventArgs) -> bool {
        let Some(entry) = self.entry(id) else {
            return false;
        };
        let Some((event_name, handler)) = entry.events.iter().find(|(n, _)| n == event).map(|(n, h)| (n.clone(), h.clone())) else {
            return false;
        };
        if self.has_observers() {
            let name = match &handler {
                Handler::Named(n) => n.clone(),
                Handler::Closure(_) => String::new(),
            };
            self.report(&crate::activity::Activity::Event { component: id, event: &event_name, handler: &name });
        }
        if let Some(trace) = self.trace.clone() {
            let name = match &handler {
                Handler::Named(n) => n.as_str(),
                Handler::Closure(_) => "",
            };
            trace(self, id, &event_name, name);
        }
        match handler {
            Handler::Closure(f) => {
                let _profile = crate::profile::scope(crate::profile::Lane::Event, || format!("{}.{event_name}", self.path_of(id)));
                self.nested(id, &event_name, |ui| f(ui, id, args))
            }
            Handler::Named(handler_name) => self.call_named(id, &event_name, &handler_name, args),
        }
    }

    /// Roda um handler contando o aninhamento; acima do limite, é um erro (a caixa de erro e o
    /// `on_exception`), e o handler não roda: quase sempre é um laço sem querer (um
    /// `on_change` que muda o próprio valor).
    fn nested(&mut self, id: ComponentId, event: &str, call: impl FnOnce(&mut Ui)) -> bool {
        let limit = self.limits.1;
        if self.event_depth >= limit {
            // Só no nível mais fundo: os de cima não repetem o erro ao voltar.
            if self.event_depth == limit {
                self.event_depth += 1;
                let message = format!("evento recursivo demais: {}.{event} ({limit} níveis)", self.path_of(id));
                self.raise_exception(crate::Exception { message, path: self.path_of(id), event: event.to_string() });
                self.event_depth -= 1;
            }
            return false;
        }
        self.event_depth += 1;
        // Um pânico no handler vira a caixa de erro (veja `exception.rs`), e o app segue.
        let depth = self.event_depth;
        // Um pânico no meio de um `without_change_events` não deixa o `on_change` desligado.
        let quiet = self.quiet_changes;
        self.running_events.push(event.to_string());
        self.guarded(id, event, call);
        self.running_events.pop();
        self.event_depth = depth - 1;
        self.quiet_changes = quiet;
        true
    }

    /// Muda os limites de segurança (o `limits` do `.aylisproj`): a profundidade da árvore de
    /// componentes e os eventos aninhados. Abaixo de 8, fica 8.
    pub fn set_limits(&mut self, tree_depth: usize, event_nesting: usize) {
        self.limits = (tree_depth.max(8), event_nesting.max(8));
    }

    /// Os limites em uso: (profundidade da árvore, eventos aninhados).
    pub fn limits(&self) -> (usize, usize) {
        self.limits
    }

    pub(crate) fn observers(&self) -> &crate::activity::Observers {
        &self.observers
    }

    pub(crate) fn observers_mut(&mut self) -> &mut crate::activity::Observers {
        &mut self.observers
    }

    /// Liga o rastro de eventos (para depurar): `f(ui, componente, evento, handler)` é
    /// chamado antes de cada evento que tem handler (`handler` vazio = closure em Rust).
    pub fn set_event_trace(&mut self, f: Option<impl Fn(&Ui, ComponentId, &str, &str) + 'static>) {
        self.trace = f.map(|f| std::rc::Rc::new(f) as std::rc::Rc<TraceFn>);
    }

    /// Chama o handler de nome `handler` como se o evento `event` de `id` tivesse
    /// disparado: o registrado para o form do componente, o geral ou, sem nenhum, o
    /// receptor de eventos (C ABI, protocolo). Devolve se alguém tratou.
    pub fn call_handler(&mut self, id: ComponentId, event: &str, handler: &str, args: &EventArgs) -> bool {
        self.call_named(id, event, handler, args)
    }

    fn call_named(&mut self, id: ComponentId, event: &str, handler_name: &str, args: &EventArgs) -> bool {
        // Primeiro o handler do form ("frmClientes.btn_salvar_click"), depois o geral:
        // dois forms podem ter handlers com o mesmo nome.
        // O nome com que o form foi criado: um frame repetido (`fraLinha2`) usa os handlers
        // do `fraLinha`.
        // Um escopo de nomes que não é o form dos handlers (a linha de um DataRepeater, uma cópia
        // do modelo) não os tem: sobe de escopo em escopo até o form que tem este handler (o
        // clique num botão da linha não chegava ao form; relatório do RTSPlayer, 10/11).
        let mut form = self.form_of(id);
        loop {
            let name = self.form_class(form).or_else(|| self.name(form)).unwrap_or_default();
            if self.handlers.get(name).is_some_and(|m| m.contains_key(handler_name)) {
                break;
            }
            match self.parent(form).map(|p| self.form_of(p)) {
                Some(up) if up != form => form = up,
                _ => {
                    form = self.form_of(id);
                    break;
                }
            }
        }
        let form_name = self.form_class(form).or_else(|| self.name(form)).unwrap_or_default();
        // O form e o método (o que a tabela do profiler mostra e leva ao código); o texto só
        // é montado com o profiler ligado.
        let _profile = crate::profile::scope(crate::profile::Lane::Handler, || {
            if form_name.is_empty() { handler_name.to_string() } else { format!("{form_name}.{handler_name}") }
        });
        let of_form = |form: &str| self.handlers.get(form).and_then(|m| m.get(handler_name)).cloned();
        // Um clone do `Rc`: o handler continua registrado enquanto roda (reentrante).
        let found = if form_name.is_empty() { None } else { of_form(form_name) }.or_else(|| of_form(""));
        if let Some(f) = found {
            return self.nested(id, event, |ui| f(ui, id, args));
        }
        if let Some(sink) = self.event_sink.clone() {
            return self.nested(id, event, |ui| sink(ui, id, event, handler_name, args));
        }
        false
    }

    /// Agenda um evento para ser entregue em [`process_events`](Self::process_events).
    pub fn queue_event(&mut self, id: ComponentId, event: &str, args: EventArgs) {
        self.pending.push((id, event.to_string(), args));
    }

    /// Entrega os eventos agendados (inclusive os agendados pelos próprios handlers), até
    /// [`EVENTS_PER_ROUND`] por chamada: o que passar disso fica na fila para a próxima volta
    /// do laço da janela, que repinta e atende o usuário no meio. Um handler que sempre
    /// reagenda o próprio evento deixa o app lento, mas não congelado.
    pub fn process_events(&mut self) {
        let mut delivered = 0;
        while !self.pending.is_empty() {
            let mut batch = std::mem::take(&mut self.pending).into_iter();
            for (id, event, args) in batch.by_ref() {
                self.count_repeat(id, &event);
                self.fire(id, &event, &args);
                delivered += 1;
                if delivered >= EVENTS_PER_ROUND {
                    break;
                }
            }
            if delivered >= EVENTS_PER_ROUND {
                // O que sobrou volta para o começo da fila, na ordem; a próxima volta continua.
                let mut left: Vec<_> = batch.collect();
                left.append(&mut self.pending);
                self.pending = left;
                if !self.pending.is_empty() {
                    self.poster().wake();
                    return;
                }
            }
        }
        self.repeat = None;
    }

    /// Há eventos esperando a próxima volta de [`process_events`](Self::process_events)?
    pub fn has_pending_events(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Conta o mesmo evento do mesmo componente seguido; em [`LOOP_WARNING`] repetições, avisa
    /// (uma vez) de um possível laço, com o caminho do evento.
    fn count_repeat(&mut self, id: ComponentId, event: &str) {
        let count = match &mut self.repeat {
            Some((last, name, count)) if *last == id && name == event => {
                *count += 1;
                *count
            }
            _ => {
                self.repeat = Some((id, event.to_string(), 1));
                1
            }
        };
        if count == LOOP_WARNING {
            let message = format!("possível laço: {}.{event} disparado {LOOP_WARNING} vezes seguidas", self.path_of(id));
            self.report(&crate::activity::Activity::Error { message: &message });
        }
    }

    /// Executa um método do componente com um [`EventCtx`] e entrega os eventos que ele
    /// disparou. É assim que a plataforma entrega mouse, teclado etc.
    ///
    /// Devolve `None` se o id não existir ou se o componente já estiver executando.
    pub fn with_component<R>(
        &mut self,
        id: ComponentId,
        f: impl FnOnce(&mut dyn Component, &mut EventCtx<'_>) -> R,
    ) -> Option<R> {
        let before = self.cb(id).map(|cb| cb.state());
        let mut component = self.entry_mut(id)?.component.take()?;
        self.call_depth += 1;
        let depth = (self.call_depth, self.event_depth);
        let result = {
            let mut ctx = EventCtx::new(self, id);
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(component.as_mut(), &mut ctx)))
        };
        (self.call_depth, self.event_depth) = (depth.0 - 1, depth.1);
        // Se o componente foi liberado durante a chamada, simplesmente é descartado. Com um
        // pânico no método, ele volta também (senão sumiria da tela).
        if let Some(entry) = self.entry_mut(id) {
            entry.component = Some(component);
        }
        let result = match result {
            Ok(r) => Some(r),
            Err(payload) => {
                let message = crate::exception::panic_message(payload.as_ref());
                self.raise_exception(crate::Exception { message, path: self.path_of(id), event: String::new() });
                None
            }
        };
        if let Some(before) = before {
            self.sync_state_triggers(id, before);
        }
        if let Some(focus) = self.pending_focus.take() {
            self.set_focus(focus);
        }
        self.process_events();
        if self.call_depth == 0 {
            self.flush_closes();
        }
        result
    }

    /// Entrega uma mensagem ao componente ([`Component::message`]). Devolve `false` se o
    /// componente não existe ou está ocupado (executando um método dele mesmo).
    pub fn send_message(&mut self, id: ComponentId, message: &dyn Any) -> bool {
        if let Some(error) = self.busy_error(id) {
            self.report_busy(&error);
            return false;
        }
        self.with_component(id, |c, ctx| c.message(ctx, message)).is_some()
    }

    /// Chama um método do componente pelo nome ([`Component::invoke`]), como
    /// `ui.invoke(desenho, "undo", &[])` ou `ui.invoke(desenho, "save", &["a.png".into()])`.
    pub fn invoke(&mut self, id: ComponentId, method: &str, args: &[PropValue]) -> std::result::Result<PropValue, String> {
        if let Some(error) = self.busy_error(id) {
            self.report_busy(&error);
            return Err(error.to_string());
        }
        let args = self.component_args(id, method, args);
        self.with_component(id, |c, ctx| c.invoke(ctx, method, &args)).unwrap_or_else(|| Err(Error::InvalidId.to_string()))
    }

    /// Os argumentos que o método declara como componente ([`PropKind::Component`]) chegam
    /// como o componente, venham como o caminho (`"frmX.lbl"`, o nome) ou como o id numérico
    /// (o que os clientes das outras linguagens mandam): o componente não precisa conferir.
    fn component_args(&self, id: ComponentId, method: &str, args: &[PropValue]) -> Vec<PropValue> {
        let kinds = self.component(id).and_then(|c| c.methods().iter().find(|m| m.name == method)).map_or(&[][..], |m| m.args);
        args.iter()
            .enumerate()
            .map(|(i, arg)| {
                if kinds.get(i) != Some(&PropKind::Component) {
                    return arg.clone();
                }
                match arg {
                    PropValue::String(path) if path.is_empty() => PropValue::Component(None),
                    PropValue::String(path) => self.find_path(path).map_or_else(|| arg.clone(), |c| PropValue::Component(Some(c))),
                    PropValue::Int(n) => {
                        let c = ComponentId::from_u64(*n as u64);
                        if self.contains(c) { PropValue::Component(Some(c)) } else { arg.clone() }
                    }
                    PropValue::Null => PropValue::Component(None),
                    _ => arg.clone(),
                }
            })
            .collect()
    }

    /// O erro de "ocupado", se o componente existe mas está fora do `Ui` (um método dele
    /// rodando).
    fn busy_error(&self, id: ComponentId) -> Option<Error> {
        self.entry(id).filter(|e| e.component.is_none()).map(|_| Error::Busy(self.path_of(id)))
    }

    /// Em modo de desenvolvimento, o acesso a um componente ocupado vai também para o log
    /// (com o caminho), para achar quem tentou.
    fn report_busy(&mut self, error: &Error) {
        if cfg!(debug_assertions) {
            let message = error.to_string();
            self.report(&crate::activity::Activity::Error { message: &message });
        }
    }

    /// Modo de design (a IDE liga no `Ui` do documento): os controles podem se mostrar de
    /// um jeito próprio para o designer (veja [`PaintCtx::design`](crate::PaintCtx::design)).
    /// Quanto o mouse fica parado até a dica aparecer, em segundos (o `HintPause` do Delphi;
    /// o relatório do Tabernáculo, C19). `None` volta ao do tema (`durations.hint`, em ms) ou
    /// ao padrão de 1 s.
    pub fn set_hint_delay(&mut self, seconds: Option<f64>) {
        self.hint_delay = seconds.map(|s| s.max(0.0));
    }

    /// Liga a pintura de foto: os estados do momento (hover, pressed, focused) não aparecem.
    /// Quem tira a foto liga antes de pintar e desliga depois.
    pub fn set_still_picture(&mut self, on: bool) {
        self.still_picture = on;
    }

    /// O atraso da dica em vigor, em segundos.
    pub fn hint_delay(&self) -> f64 {
        self.hint_delay
            .or_else(|| self.theme.durations.get("hint").map(|ms| *ms as f64 / 1000.0))
            .unwrap_or(crate::input::HINT_DELAY)
    }

    pub fn set_design_mode(&mut self, on: bool) {
        self.design_mode = on;
        self.request_repaint();
    }

    /// No designer, quais componentes escondidos (`visible = false`) aparecem: os de
    /// `selected` e os que contêm um deles (para editar o que só aparece em certas
    /// situações, como um popup); com `show_all`, todos. Os outros escondidos somem, como
    /// no app, e não ocupam lugar. Os que aparecem ficam translúcidos.
    pub fn set_design_reveal(&mut self, selected: &[ComponentId], show_all: bool) {
        self.design_reveal = selected.to_vec();
        self.design_show_hidden = show_all;
        let mut stack = self.roots().to_vec();
        while let Some(id) = stack.pop() {
            stack.extend(self.children(id).iter().copied());
            if self.cb(id).is_some_and(|cb| cb.design_hidden) {
                let shown = self.design_revealed(id);
                if let Some(cb) = self.cb_mut(id) {
                    cb.set_visible(shown);
                }
            }
        }
        self.invalidate();
    }

    /// Um escondido que aparece no designer: todos aparecem, ou ele é (ou contém) um dos
    /// selecionados.
    fn design_revealed(&self, id: ComponentId) -> bool {
        self.design_show_hidden
            || self.design_reveal.iter().any(|&s| {
                let mut cur = Some(s);
                while let Some(c) = cur {
                    if c == id {
                        return true;
                    }
                    cur = self.parent(c);
                }
                false
            })
    }

    pub fn design_mode(&self) -> bool {
        self.design_mode
    }

    /// O designer mudou o documento (um componente novo, excluído, movido, uma propriedade
    /// de um filho): cada componente de `root` para baixo recebe a mensagem
    /// [`DesignChanged`] para refazer o que guarda (a grade, as colunas).
    pub fn design_changed(&mut self, root: ComponentId) {
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            stack.extend(self.children(id).iter().copied());
            self.send_message(id, &DesignChanged);
        }
    }

    /// Pede para fechar a janela e encerrar o aplicativo (como `Close` no form principal).
    pub fn request_close(&mut self) {
        self.close_requested = true;
    }

    /// Alguém pediu para fechar (a plataforma consulta isto).
    pub fn close_requested(&self) -> bool {
        self.close_requested
    }

    /// Copia o componente `source` e todos os filhos para dentro de `parent`: mesma classe,
    /// as propriedades e os handlers por nome (os de closure não vão). A cópia é um escopo
    /// de nomes próprio (os filhos repetem os nomes do original sem conflito) e não leva o
    /// nome da raiz. É o que a `VirtualList` usa para multiplicar o modelo de um item.
    pub fn clone_component(&mut self, source: ComponentId, parent: Option<ComponentId>) -> Result<ComponentId> {
        self.clone_node(source, parent, false)
    }

    fn clone_node(&mut self, source: ComponentId, parent: Option<ComponentId>, named: bool) -> Result<ComponentId> {
        let class = self.class_name(source).ok_or(Error::InvalidId)?.to_string();
        let id = self.create(&class, parent)?;
        if named {
            if let Some(name) = self.name(source).filter(|n| !n.is_empty()).map(str::to_string) {
                let _ = self.set_name(id, &name);
            }
        }
        // A raiz da cópia é um escopo antes dos filhos ganharem nome.
        if !named || self.entry(source).is_some_and(|e| e.scope) {
            self.set_name_scope(id, true);
        }
        // `align` e `anchors` por último: a posição e o tamanho, gravados depois, os
        // desfariam.
        let mut infos = self.prop_infos(source);
        infos.sort_by_key(|i| matches!(i.name, "align" | "anchors"));
        for info in infos {
            if info.read_only || info.runtime || info.name == "name" {
                continue;
            }
            if let Ok(value) = self.get_prop(source, info.name) {
                let _ = self.set_prop_quiet(id, info.name, value);
            }
        }
        for child in self.children(source).to_vec() {
            self.clone_node(child, Some(id), true)?;
        }
        Ok(id)
    }

    /// Todos os componentes vivos, em qualquer ordem.
    pub fn components(&self) -> impl Iterator<Item = ComponentId> + '_ {
        self.slots.iter().enumerate().filter_map(|(index, slot)| {
            slot.entry.as_ref().map(|_| ComponentId { index: index as u32, generation: slot.generation })
        })
    }

    /// Simula um clique (como `Button1.Click`).
    pub fn click(&mut self, id: ComponentId) -> bool {
        self.with_component(id, |c, ctx| c.click(ctx)).is_some()
    }

    // =====================================================================
    // Depuração
    // =====================================================================

    /// Árvore em texto, com classe, nome, alinhamento e posição real de cada componente.
    pub fn dump(&self, id: ComponentId) -> String {
        let mut out = String::new();
        self.dump_into(id, 0, &mut out);
        out
    }

    fn dump_into(&self, id: ComponentId, depth: usize, out: &mut String) {
        let Some(entry) = self.entry(id) else { return };
        let class = entry.component.as_deref().map_or("?", |c| c.class_name());
        let _ = write!(out, "{:indent$}{class}", "", indent = depth * 2);
        if !entry.name.is_empty() {
            let _ = write!(out, " {}", entry.name);
        }
        if let Some(cb) = self.cb(id) {
            let b = cb.bounds;
            let _ = write!(
                out,
                " [{:?}] x={} y={} w={} h={}",
                cb.align(),
                b.x,
                b.y,
                b.width,
                b.height
            );
        }
        out.push('\n');
        for &c in &entry.children {
            self.dump_into(c, depth + 1, out);
        }
    }
}

fn too_deep(limit: usize) -> Error {
    Error::invalid(format!("profundidade máxima de componentes ({limit} níveis)"))
}

/// Identificador: letra ASCII ou `_`, depois letras, dígitos ou `_` (nomes de componente e de
/// handler, que viram código em todas as linguagens).
pub fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn bounds_field(b: &Bounds, field: &str) -> Option<f32> {
    match field {
        "left" => Some(b.left),
        "top" => Some(b.top),
        "right" => Some(b.right),
        "bottom" => Some(b.bottom),
        _ => None,
    }
}

fn bounds_field_mut<'a>(b: &'a mut Bounds, field: &str) -> Option<&'a mut f32> {
    match field {
        "left" => Some(&mut b.left),
        "top" => Some(&mut b.top),
        "right" => Some(&mut b.right),
        "bottom" => Some(&mut b.bottom),
        _ => None,
    }
}

/// Tenta gravar na camada e, se não for dela, nas camadas base.
fn set_in_chain(c: &mut dyn Component, name: &str, value: PropValue) -> Result<()> {
    match c.set_prop(name, value.clone()) {
        Err(Error::UnknownProperty(_)) => match c.inner_mut() {
            Some(inner) => set_in_chain(inner, name, value),
            None => Err(Error::UnknownProperty(name.to_string())),
        },
        other => other,
    }
}

fn find_layer<T: Component>(c: &dyn Component) -> Option<&T> {
    let any: &dyn Any = c;
    if let Some(t) = any.downcast_ref::<T>() {
        return Some(t);
    }
    c.inner().and_then(find_layer::<T>)
}

fn find_layer_mut<T: Component>(c: &mut dyn Component) -> Option<&mut T> {
    if (&*c as &dyn Any).is::<T>() {
        let any: &mut dyn Any = c;
        return any.downcast_mut::<T>();
    }
    c.inner_mut().and_then(find_layer_mut::<T>)
}

// =====================================================================
// Automação: os itens desenhados pelos controles
// =====================================================================

impl Ui {
    /// Os itens que o controle desenha sozinho (abas, linhas de lista, itens de menu,
    /// células): o que o leitor de tela e a automação do sistema veem dentro dele.
    pub fn access_items(&self, id: ComponentId) -> Vec<crate::automation::AccessItem> {
        self.component(id).map_or_else(Vec::new, |c| c.access_items(self, id))
    }

    /// Clica ou seleciona um item do controle (sem evento de mouse, como a automação do
    /// sistema faz). Devolve `false` se o controle não fez nada.
    pub fn access_action(&mut self, id: ComponentId, index: usize, action: crate::automation::ItemAction) -> bool {
        let done = self.with_component(id, |c, ctx| c.access_action(ctx, index, action)).unwrap_or(false);
        if done {
            self.request_repaint();
        }
        done
    }

    /// Clica no item com o texto `name` (sem diferenciar maiúsculas): a aba "Clientes", a
    /// linha "São Paulo", o item de menu "Salvar". Para testes e automação.
    pub fn click_item(&mut self, id: ComponentId, name: &str) -> bool {
        let found = self.access_items(id).iter().position(|i| i.name.trim().eq_ignore_ascii_case(name.trim()));
        found.is_some_and(|index| self.access_action(id, index, crate::automation::ItemAction::Click))
    }
}

impl Ui {
    /// A fonte com que o controle é desenhado: a do estilo dele no tema (a `default` se o
    /// estilo não der uma) com a fonte própria dele e a dos ancestrais por cima (o
    /// `ParentFont`). Para medir texto fora da pintura (a largura de um menu, o tamanho de
    /// um painel) com a mesma fonte do desenho.
    /// O espaço interno que o tema dá ao controle (o `padding` do estilo dele): onde o
    /// texto é desenhado.
    /// A altura de linha do estilo do controle (`line_height`, em vezes a fonte; 0: a da fonte).
    pub fn style_line_height(&self, id: ComponentId) -> f32 {
        let theme = self.theme_of(id);
        match self.component(id).and_then(|c| c.control().map(|cb| (c, cb))) {
            Some((component, cb)) => {
                let mut classes = Vec::new();
                component.class_chain(&mut classes);
                self.mixed_theme(theme, &classes).resolve(&classes, cb.style(), cb.state()).line_height
            }
            None => 0.0,
        }
    }

    pub fn style_padding(&self, id: ComponentId) -> crate::geometry::Bounds {
        let theme = self.theme_of(id);
        match self.component(id).and_then(|c| c.control().map(|cb| (c, cb))) {
            Some((component, cb)) => {
                let mut classes = Vec::new();
                component.class_chain(&mut classes);
                self.mixed_theme(theme, &classes).resolve(&classes, cb.style(), cb.state()).padding
            }
            None => Default::default(),
        }
    }

    pub fn font_of(&self, id: ComponentId) -> crate::canvas::FontSpec {
        let theme = self.theme_of(id);
        let mut font = match self.component(id).and_then(|c| c.control().map(|cb| (c, cb))) {
            Some((component, cb)) => {
                let mut classes = Vec::new();
                component.class_chain(&mut classes);
                self.mixed_theme(theme, &classes).resolve(&classes, cb.style(), cb.state()).font
            }
            None => theme.fonts.get("default").cloned().unwrap_or_default(),
        };
        let mut chain = Vec::new();
        let mut cur = Some(id);
        while let Some(c) = cur {
            if let Some(cb) = self.control(c) {
                chain.push(cb.font().clone());
            }
            cur = self.parent(c);
        }
        // `over` dá preferência a quem chama: do controle para o form, o mais perto vence.
        let effective = chain.iter().fold(crate::component::FontOverride::default(), |acc, f| acc.over(f));
        effective.apply(&mut font);
        font.family = self.resolve_font(&font.family);
        // A escala de fonte da vista ativa do form.
        font.size *= self.control(id).map_or(1.0, |cb| cb.text_scale());
        font
    }
}

impl Ui {
    /// O estado que um crate guarda neste `Ui` (um valor por tipo, criado na primeira vez):
    /// o protocolo remoto guarda o canal e os eventos sem `done`, por tela (duas telas no mesmo
    /// processo não se misturam).
    pub fn extension<T: Default + 'static>(&mut self) -> &mut T {
        self.extensions
            .entry(std::any::TypeId::of::<T>())
            .or_insert_with(|| Box::new(T::default()))
            .downcast_mut::<T>()
            .expect("o tipo é a chave")
    }

    /// O estado do tipo `T`, se já foi criado.
    pub fn extension_ref<T: 'static>(&self) -> Option<&T> {
        self.extensions.get(&std::any::TypeId::of::<T>()).and_then(|b| b.downcast_ref::<T>())
    }
}

/// O alinhamento `align` decide a largura (`width`) ou a altura do controle.
fn stretches(align: crate::align::Align, width: bool) -> bool {
    use crate::align::Align;
    match align {
        Align::None | Align::Center => false,
        Align::VertCenter | Align::Horizontal => width,
        Align::HorzCenter | Align::Vertical => !width,
        Align::Top | Align::Bottom | Align::MostTop | Align::MostBottom => width,
        Align::Left | Align::Right | Align::MostLeft | Align::MostRight => !width,
        _ => true,
    }
}
