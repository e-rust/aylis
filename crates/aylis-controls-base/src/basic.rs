//! Controles simples: Form, Layout, Frame, Rectangle, Panel, Label, Button e ProgressBar.

use aylis_core::{
    Bounds, Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent, LayerCaching,
    FormPresent, GradientStop, ModalResult, PaintCtx, PointF, PropEnum, RectF, Stroke, TextAlign, TextCase, TextStyle, Transition, Trimming, properties,
};

/// Onde fica o ícone de um botão em relação ao texto.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconPosition {
    #[default]
    Left,
    Right,
    Top,
}

impl PropEnum for IconPosition {
    const NAMES: &'static [&'static str] = &["left", "right", "top"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Left, Self::Right, Self::Top].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

use crate::leaf_control;

/// Onde a janela do form abre.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormPosition {
    /// No centro do form dono (o que estava ativo) ou, sem dono, da tela.
    #[default]
    Center,
    /// No centro da tela.
    ScreenCenter,
    /// Em `left`/`top`, coordenadas da tela.
    Designed,
    /// Onde o sistema escolher.
    Default,
    /// Num canto da área útil do monitor (o de `monitor` ou o principal), sem a barra de
    /// tarefas.
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl PropEnum for FormPosition {
    const NAMES: &'static [&'static str] = &["center", "screen_center", "designed", "default", "top_left", "top_right", "bottom_left", "bottom_right"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Center, Self::ScreenCenter, Self::Designed, Self::Default, Self::TopLeft, Self::TopRight, Self::BottomLeft, Self::BottomRight].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Como a janela do form fica, inclusive em tela
/// cheia. Vale ao abrir e, com o app rodando, quando muda.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowState {
    #[default]
    Normal,
    Maximized,
    Minimized,
    /// Tela cheia: sem bordas nem barra de título, ocupando o monitor da janela.
    FullScreen,
}

impl PropEnum for WindowState {
    const NAMES: &'static [&'static str] = &["normal", "maximized", "minimized", "full_screen"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Normal, Self::Maximized, Self::Minimized, Self::FullScreen].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Borda da janela.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BorderStyle {
    /// Redimensionável, com maximizar.
    #[default]
    Sizeable,
    /// Tamanho fixo (diálogos).
    Single,
    /// Sem borda nem barra de título.
    None,
    /// Sem a barra de título do sistema, para uma `TitleBar` própria; continua
    /// redimensionável pelas bordas.
    Custom,
}

/// Os cantos da janela, desenhados pelo sistema (com a borda suave e a sombra dele).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowCorners {
    /// O do sistema: arredondados como as outras janelas, também com `border_style = custom`
    /// (a barra própria é uma janela comum); com `none`, retos.
    #[default]
    Default,
    /// Arredondados.
    Round,
    /// Arredondados, com o raio pequeno (o dos menus do Windows 11).
    Small,
    /// Retos.
    Square,
}

impl PropEnum for WindowCorners {
    const NAMES: &'static [&'static str] = &["default", "round", "small", "square"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Round, Self::Small, Self::Square].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// A forma da janela do form (`window_shape`), recortada pelo Aylis com a borda suave.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowShape {
    /// O retângulo (com os cantos de `window_corners`).
    #[default]
    Rectangle,
    /// Os cantos arredondados com o raio `shape_radius`.
    Rounded,
    /// A elipse do tamanho do form (um círculo, se ele é quadrado).
    Ellipse,
    /// Um contorno qualquer (`shape_path`, como o `d` do SVG), esticado para o tamanho do form.
    Path,
}

impl PropEnum for WindowShape {
    const NAMES: &'static [&'static str] = &["rectangle", "rounded", "ellipse", "path"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Rectangle, Self::Rounded, Self::Ellipse, Self::Path].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

impl PropEnum for BorderStyle {
    const NAMES: &'static [&'static str] = &["sizeable", "single", "none", "custom"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Sizeable, Self::Single, Self::None, Self::Custom].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Janela. Pinta o fundo com o estilo `Form` do tema.
///
/// Abrir e fechar: [`Ui::show`](aylis_core::Ui::show), [`Ui::show_modal`](aylis_core::Ui::show_modal)
/// e [`Ui::close`](aylis_core::Ui::close). Um form também pode ser embutido em outro
/// (`ui.set_parent(form, Some(painel))`).
pub struct Form {
    pub base: ControlBase,
    /// O título da janela (na barra de título e na barra de tarefas).
    pub caption: String,
    /// Onde a janela abre: no centro do dono ou da tela, em `left`/`top`, onde o sistema
    /// escolher ou num canto do monitor.
    pub position: FormPosition,
    /// A borda da janela: redimensionável, de tamanho fixo, sem borda nem título (`none`) ou
    /// sem a barra do sistema, para uma `TitleBar` própria (`custom`).
    pub border_style: BorderStyle,
    /// Os cantos da janela (Windows 11 e macOS; nas outras, sem efeito): útil com a barra de
    /// título própria (`border_style = custom`), que fica com os cantos de uma janela comum.
    pub window_corners: WindowCorners,
    /// A janela deixa ver o que está atrás onde o form não é opaco: o fundo com alfa (`fill`
    /// `#RRGGBBAA`, a semitransparência), o que fica fora de `window_shape` e os pixels
    /// transparentes da pintura. Com o `blur` do `WindowSettings`, o que está atrás sai
    /// desfocado (o vidro fosco). Desligado (o padrão), a janela é opaca. Windows com o
    /// Direct2D (o padrão), macOS e Linux com compositor; no Windows com o Skia, no celular e no
    /// navegador, sem efeito (a janela fica opaca e retangular).
    pub transparent: bool,
    /// A forma da janela: o retângulo, os cantos arredondados (`rounded`, com `shape_radius` ou
    /// um raio por canto em `shape_radii`), a elipse (`ellipse`: o form quadrado vira um círculo)
    /// ou um contorno qualquer (`path`, com `shape_path`). A borda é suavizada (antialiasing),
    /// os filhos ficam dentro dela e os cliques fora dela vão para o que está atrás. Fora do
    /// retângulo, a janela fica transparente e sem a moldura do sistema (como `border_style =
    /// none`, sem redimensionar pela borda), e o form aparece com a forma também no designer.
    pub window_shape: WindowShape,
    /// O raio dos cantos com `window_shape = rounded` (unidades do form).
    pub shape_radius: f32,
    /// Um raio por canto com `window_shape = rounded`, na ordem do CSS: cima-esquerda,
    /// cima-direita, baixo-direita, baixo-esquerda (`"24,24,0,0"`: só os de cima). Todos zero (o
    /// padrão): vale o `shape_radius`.
    pub shape_radii: aylis_core::Bounds,
    /// O contorno com `window_shape = path`, no formato do `d` do SVG (`M 100 0 L 200 100 L 100
    /// 200 L 0 100 Z`: um losango). Os limites do contorno viram os do form: as coordenadas
    /// podem estar em qualquer escala, e o contorno acompanha quando o form muda de tamanho.
    /// Inválido ou vazio: o retângulo.
    pub shape_path: String,
    /// O `shape_path` já lido (o texto e o caminho), para não ler a cada pintura.
    shape_cache: std::cell::RefCell<Option<(String, Option<std::sync::Arc<aylis_core::Path>>)>>,
    /// Normal, maximizada, minimizada ou em tela cheia (`full_screen`). No celular e no
    /// navegador o form já ocupa a tela toda.
    pub window_state: WindowState,
    /// Celular e navegador (os forms se empilham): como o form entra ao abrir e sai ao
    /// fechar (`slide_left` como uma tela nova do Android/iOS, `slide_up` como uma folha,
    /// `fade`). No desktop cada form é uma janela e isto não vale.
    pub transition: Transition,
    /// As camadas automáticas neste form: as páginas de uma transição e o contêiner que
    /// recorta os filhos com o lugar ou a transparência animando viram uma imagem guardada,
    /// que anda sem repintar os filhos (mais rápido, e a memória das imagens enquanto andam).
    /// `project`: o padrão do projeto (`image_cache.auto_layers`); `on`/`off` trocam só neste
    /// form. O `cache_layer` pedido num controle vale sempre.
    pub layer_cache: LayerCaching,
    /// Até quantos KB um quadro de câmera ou de vídeo reaproveita o bitmap do quadro anterior
    /// na placa nesta janela, em vez de criar outro. `-1` (o padrão): o do projeto
    /// (`image_cache.frame_reuse_kb`, que o `RenderSettings` troca em execução); `0`: sempre
    /// cria; acima de 32768, ajustado. Vale no Direct2D (Windows), no quadro seguinte à troca;
    /// no Skia e no navegador, sem efeito (eles não reaproveitam).
    pub frame_reuse_kb: i64,
    /// Sempre por cima das outras janelas.
    pub topmost: bool,
    /// Sempre por baixo das outras janelas, como um fundo de tela (com `topmost` também, vale
    /// o `topmost`).
    pub bottommost: bool,
    /// Enquanto o form está aberto, o sistema não dorme (`system`) ou nem apaga a tela
    /// (`display`). Vale o maior pedido entre os forms abertos. Windows; nas outras
    /// plataformas, sem efeito por enquanto.
    pub keep_awake: KeepAwake,
    /// Prende o mouse na janela (`window`) ou no monitor dela (`monitor`) enquanto ela está
    /// ativa; trocar de janela (Alt+Tab) solta.
    pub confine_cursor: CursorConfine,
    /// A transparência da janela inteira, com a moldura (1 = opaca, 0,5 = metade). Windows;
    /// nas outras plataformas, sem efeito. Para o fundo transparente com os controles opacos,
    /// use `transparent`.
    pub window_opacity: f32,
    /// A tela cheia sem bordas (o padrão) ou exclusiva, com o modo de vídeo pedido.
    pub full_screen_mode: FullScreenMode,
    /// A largura em pixels do modo de vídeo da tela cheia exclusiva (0: a atual do monitor).
    /// Vale o modo do monitor mais perto da largura, da altura e da taxa pedidas.
    pub video_width: i64,
    /// A altura em pixels do modo de vídeo da tela cheia exclusiva (0: a atual do monitor).
    pub video_height: i64,
    /// A taxa de atualização em Hz da tela cheia exclusiva (0: a atual do monitor).
    pub video_refresh: f32,
    /// Aparece na barra de tarefas (desligue para menus e janelas de ferramenta).
    pub show_in_taskbar: bool,
    /// O controle que recebe o foco quando o form abre.
    pub active_control: Option<ComponentId>,
    /// A ordem do Tab no form inteiro: nomes separados por `;` (`edtNome;edtEmail;btnSalvar`).
    /// Pula entre níveis (de um campo num painel para outro num grupo); os controles fora
    /// da lista vêm depois, na ordem natural. Vazio: a ordem natural (a da árvore).
    pub tab_order: String,
    /// As teclas que fazem o papel do Tab (`"Enter"`, `"Enter;Down"`), como o Enter que pula
    /// de campo nos sistemas de cadastro. O controle que usa a tecla (o Enter num Memo) fica
    /// com ela.
    pub tab_keys: String,
    /// As que fazem o papel do Shift+Tab (`"Shift+Enter;Up"`).
    pub back_tab_keys: String,
    /// Os caracteres liberados, bloqueados e só os liberados, para todos os campos do form
    /// (o campo decide primeiro; `chars_override` faz o form mandar). Faixas: `0-9`, `a-z`.
    pub allowed_chars: String,
    /// Os caracteres que os campos do form não aceitam (digitados ou colados); faixas como
    /// `a-z`. Vazio: nenhum.
    pub blocked_chars: String,
    /// Só os caracteres de `allowed_chars` entram nos campos do form.
    pub only_allowed: bool,
    /// O filtro de caracteres do form manda nos campos de dentro (sem ele, o do campo decide
    /// primeiro).
    pub chars_override: bool,
    /// Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a
    /// mesma herança dos filtros.
    pub char_map: String,
    /// O form recebe as teclas antes do controle com foco (`on_key_down` e `on_key_press`).
    pub key_preview: bool,
    /// O zoom deste form (1 = normal), vezes o do app ([`Ui::set_zoom`]): tudo nele fica
    /// maior ou menor e o layout se refaz no espaço da janela.
    pub zoom: f32,
    /// Em que monitor a janela abre (o índice do `Screen`; -1 = o do dono ou o principal). Com
    /// `position = designed`, o `left`/`top` contam do canto dele.
    pub monitor: i64,
    /// Lembra onde a janela estava (o monitor, a posição, o tamanho e se estava maximizada)
    /// e reabre igual, num arquivo do app; se aquele monitor sumiu, abre como se não houvesse.
    pub remember_placement: bool,
    /// A placa de vídeo da janela (vazio: a do projeto): `monitor` troca para a placa do
    /// monitor em que ela está (Windows e Linux com Wayland); `alto-desempenho`, `economia` ou parte do nome fixam uma.
    pub gpu: String,
    /// Como o quadro chega à janela no Windows (`project`: o do projeto): `window`, `composition`
    /// ou `composed`, sempre composta pelo DWM, fora do plano de overlay (o form em tela cheia
    /// por cima do resto, onde o cursor congelava em placas com MPO).
    pub present: FormPresent,
    /// Quem junta as camadas com `blend_mode` ao fundo no Direct2D (`project`: o do projeto):
    /// `direct2d`, o efeito do Direct2D, ou `direct3d`, o shader do Aylis em Direct3D 11 (a base
    /// para o app que precisa de controle total da placa). O resultado é o mesmo.
    pub blend_compositor: aylis_core::gpu::FormBlendCompositor,
    /// O form maior que um monitor vira uma janela por monitor, cada uma na placa de vídeo do
    /// seu (Windows; nas outras, sem efeito). Sem isso, com monitores em placas diferentes, o
    /// sistema copia cada quadro de uma placa para a outra. Com `window_state = full_screen`,
    /// o form cobre todos os monitores (um telão).
    pub span_monitors: bool,
    /// 1 unidade do form = 1 pixel físico da tela, sem a escala do monitor nem o zoom, em
    /// todas as janelas dele (num form em vários monitores, em todos os pedaços, mesmo com
    /// escalas diferentes). Um clique no pixel (x, y) chega como (x, y) do form (a partir do
    /// canto dele; com `full_screen` e `span_monitors`, do retângulo de todos os monitores), e
    /// uma imagem do tamanho do form em pixels (o print da tela) bate pixel a pixel. Mudar com
    /// a janela aberta não a move: o form passa a ter o tamanho dela na escala nova.
    pub physical_pixels: bool,
    /// Mostrar a janela dá o foco a ela (desligado: aparece sem tirar o foco de quem tem, como
    /// uma projeção que não deve roubar o teclado do console).
    pub activate_on_show: bool,
    /// Mostrar ou trazer para a frente toma o foco mesmo quando o app não recebeu a última
    /// entrada (Windows): o form aberto por um atalho global, de um timer ou da bandeja. O
    /// Windows só deixa ir para a frente quem recebeu o último clique ou tecla; sem isto, a
    /// janela aparece atrás, com o botão piscando, e o teclado (o Esc) vai para o app de trás.
    /// Nas outras plataformas, sem efeito (o pedido normal de foco).
    pub force_foreground: bool,
    /// O que fechar faz com o form: esconde (pode abrir
    /// de novo) ou libera (some da memória, depois de fechar).
    pub close_action: CloseAction,
    /// Quadros por segundo da janela (0 = a taxa do monitor dela). Um telão de 60 Hz ao lado
    /// de um monitor de 144 Hz não precisa acompanhar o mais rápido.
    pub frame_rate: f32,
    /// Pinturas por segundo, no máximo, do que muda sem a entrada do usuário (quadros de
    /// vídeo, timers, animações); 0 = desligado, cada mudança pinta na volta seguinte. Ligado,
    /// as células de uma parede de vídeos que mudaram desde a última pintura são pintadas
    /// juntas, no máximo esse tanto de vezes por segundo, em vez de uma pintura por quadro que
    /// chega. O mouse, o teclado, a roda e o toque (e o que eles puxam logo depois, como o
    /// deslize da rolagem) continuam pintando na hora; parado, nenhum relógio. Diferente do
    /// `frame_rate`, que limita tudo, a entrada inclusive.
    pub max_paint_rate: f32,
    /// Celular: a orientação da tela enquanto este form está à frente (`project`: a do
    /// projeto). Fechá-lo devolve a do form de baixo; mudar com o app aberto gira na hora. No
    /// desktop e no navegador, sem efeito.
    pub orientation: aylis_core::FormOrientation,
    /// Onde o form fica embutido (um painel, uma faixa de abas...); vazio: na janela dele.
    /// Mudar com o app rodando entra ou sai: aberto numa janela, ela some sem fechar o form
    /// (`on_dock`); solto, ele volta a ter janela, com o tamanho de antes (`on_undock`). Numa
    /// faixa de abas, ele vira uma aba com o título dele. O form principal e um modal não
    /// entram.
    pub host: Option<ComponentId>,
    /// A janela do form entra numa faixa de abas com `accept_forms` quando arrastada pela
    /// barra de título até ela (como as do navegador); embutido numa faixa, puxar a aba dele
    /// para fora o solta. Windows (nas outras plataformas, só pelo `host`).
    pub dockable: bool,
    /// Onde ele está embutido agora (o `host` pedido, depois de entrar).
    pub(crate) docked_in: Option<ComponentId>,
    /// O tamanho de janela de antes de entrar, para sair igual.
    pub(crate) undocked_size: Option<(f32, f32)>,
}

/// O que o form aberto pede para o sistema não desligar (`keep_awake`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KeepAwake {
    /// Nada: o sistema apaga a tela e dorme como de costume (o padrão).
    #[default]
    Off,
    /// O computador não dorme, mas a tela pode apagar (um download, um processamento).
    System,
    /// Nem o computador dorme nem a tela apaga (um painel, uma tela de parede, um vídeo).
    Display,
}

impl PropEnum for KeepAwake {
    const NAMES: &'static [&'static str] = &["off", "system", "display"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Off, Self::System, Self::Display].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Onde o mouse fica preso enquanto a janela do form está ativa (`confine_cursor`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CursorConfine {
    /// Solto (o padrão).
    #[default]
    None,
    /// Dentro da área da janela.
    Window,
    /// Dentro do monitor da janela (Windows; nas outras plataformas, como `window`).
    Monitor,
}

impl PropEnum for CursorConfine {
    const NAMES: &'static [&'static str] = &["none", "window", "monitor"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::None, Self::Window, Self::Monitor].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Como é a tela cheia (`full_screen_mode`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FullScreenMode {
    /// Uma janela sem bordas do tamanho do monitor, sem mudar o modo de vídeo (o padrão: troca
    /// de janela na hora, e os outros monitores e janelas continuam como estão).
    #[default]
    Borderless,
    /// O monitor passa para o modo de vídeo pedido (`video_width`, `video_height`,
    /// `video_refresh`; zero: o atual dele) enquanto o form está em tela cheia, como um jogo.
    Exclusive,
}

impl PropEnum for FullScreenMode {
    const NAMES: &'static [&'static str] = &["borderless", "exclusive"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Borderless, Self::Exclusive].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// O que fechar faz com o form (`close_action`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CloseAction {
    /// Esconde: o form continua existindo e pode abrir de novo (o padrão).
    #[default]
    Hide,
    /// Libera o form (e os componentes dele) depois de fechar.
    Free,
}

impl PropEnum for CloseAction {
    const NAMES: &'static [&'static str] = &["hide", "free"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Hide, Self::Free].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

impl Default for Form {
    fn default() -> Self {
        Self::new("", 640.0, 480.0)
    }
}

impl Form {
    /// A largura da área cliente: no Aylis, o `width` do form já é
    /// a área de dentro, sem a moldura e a barra de título do sistema.
    pub fn client_width(&self) -> f32 {
        self.base.width()
    }

    pub fn set_client_width(&mut self, width: f32) {
        let height = self.base.height();
        self.base.set_size(width.max(0.0), height);
    }

    /// A altura da área cliente, sem a barra de título.
    pub fn client_height(&self) -> f32 {
        self.base.height()
    }

    pub fn set_client_height(&mut self, height: f32) {
        let width = self.base.width();
        self.base.set_size(width, height.max(0.0));
    }
}

impl Form {
    pub fn new(caption: impl Into<String>, width: f32, height: f32) -> Self {
        Self {
            base: ControlBase::new(width, height),
            caption: caption.into(),
            position: FormPosition::default(),
            border_style: BorderStyle::default(),
            window_corners: WindowCorners::Default,
            transparent: false,
            window_shape: WindowShape::Rectangle,
            shape_radius: 16.0,
            shape_radii: aylis_core::Bounds::default(),
            shape_path: String::new(),
            shape_cache: Default::default(),
            window_state: WindowState::Normal,
            transition: Transition::None,
            layer_cache: LayerCaching::Project,
            frame_reuse_kb: -1,
            topmost: false,
            bottommost: false,
            keep_awake: KeepAwake::Off,
            confine_cursor: CursorConfine::None,
            window_opacity: 1.0,
            full_screen_mode: FullScreenMode::Borderless,
            video_width: 0,
            video_height: 0,
            video_refresh: 0.0,
            show_in_taskbar: true,
            active_control: None,
            tab_order: String::new(),
            tab_keys: String::new(),
            back_tab_keys: String::new(),
            allowed_chars: String::new(),
            blocked_chars: String::new(),
            only_allowed: false,
            chars_override: false,
            char_map: String::new(),
            key_preview: false,
            zoom: 1.0,
            monitor: -1,
            remember_placement: false,
            gpu: String::new(),
            present: FormPresent::Project,
            blend_compositor: aylis_core::gpu::FormBlendCompositor::Project,
            span_monitors: false,
            physical_pixels: false,
            activate_on_show: true,
            force_foreground: false,
            close_action: CloseAction::Hide,
            frame_rate: 0.0,
            max_paint_rate: 0.0,
            orientation: aylis_core::FormOrientation::Project,
            host: None,
            dockable: false,
            docked_in: None,
            undocked_size: None,
        }
    }
}

impl Component for Form {
    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    /// O fundo do form é um retângulo sólido: opaco se a cor é (embutido, fica transparente).
    fn opaque(&self, style: &aylis_core::ResolvedStyle) -> bool {
        style.fill.a == 255 && self.window_shape == WindowShape::Rectangle
    }

    leaf_control!("Form");

    fn clip_shape(&self) -> Option<aylis_core::ClipShape> {
        use aylis_core::ClipShape;
        match self.window_shape {
            WindowShape::Rectangle => None,
            WindowShape::Rounded if self.shape_radii != aylis_core::Bounds::default() => Some(ClipShape::Corners(self.shape_radii)),
            WindowShape::Rounded => Some(ClipShape::RoundRect(self.shape_radius.max(0.0))),
            WindowShape::Ellipse => Some(ClipShape::Ellipse),
            WindowShape::Path => {
                let mut cache = self.shape_cache.borrow_mut();
                if cache.as_ref().is_none_or(|(text, _)| *text != self.shape_path) {
                    let path = aylis_core::Path::from_svg(&self.shape_path).filter(|p| !p.is_empty()).map(std::sync::Arc::new);
                    *cache = Some((self.shape_path.clone(), path));
                }
                cache.as_ref().and_then(|(_, p)| p.clone()).map(ClipShape::Path)
            }
        }
    }

    properties! {
        caption: String => "Appearance",
        position: FormPosition => "Layout",
        border_style: BorderStyle => "Appearance",
        window_corners: WindowCorners => "Appearance",
        transparent: bool => "Appearance",
        window_shape: WindowShape => "Appearance",
        shape_radius: f32 => "Appearance",
        shape_radii: aylis_core::Bounds => "Appearance",
        shape_path: String => "Appearance",
        window_state: WindowState => "Layout",
        transition: Transition => "Behavior",
        layer_cache: LayerCaching => "Behavior",
        frame_reuse_kb: i64 => "Behavior",
        topmost: bool => "Behavior",
        bottommost: bool => "Behavior",
        keep_awake: KeepAwake => "Behavior",
        confine_cursor: CursorConfine => "Behavior",
        window_opacity: f32 => "Appearance",
        full_screen_mode: FullScreenMode => "Layout",
        video_width: i64 => "Layout",
        video_height: i64 => "Layout",
        video_refresh: f32 => "Layout",
        show_in_taskbar: bool => "Behavior",
        active_control: Option<ComponentId> => "Behavior",
        tab_order: String as List => "Behavior",
        tab_keys: String => "Keyboard",
        back_tab_keys: String => "Keyboard",
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        chars_override: bool => "Keyboard",
        char_map: String => "Keyboard",
        key_preview: bool => "Keyboard",
        zoom: f32 => "Appearance",
        monitor: i64 => "Layout",
        remember_placement: bool => "Behavior",
        gpu: String => "Behavior",
        present: FormPresent => "Behavior",
        blend_compositor: aylis_core::gpu::FormBlendCompositor => "Behavior",
        span_monitors: bool => "Layout",
        physical_pixels: bool => "Layout",
        activate_on_show: bool => "Behavior",
        force_foreground: bool => "Behavior",
        close_action: CloseAction => "Behavior",
        frame_rate: f32 => "Behavior",
        max_paint_rate: f32 => "Behavior",
        orientation: aylis_core::FormOrientation => "Layout",
        host: Option<ComponentId> => "Behavior",
        dockable: bool => "Behavior";
        events: on_show: (), on_window_created: (), on_close: (), on_close_query, on_back, on_activate: (), on_deactivate: (), on_minimize: (), on_restore: (), on_navigate: String, on_gpu_change: String, on_dock: Option<ComponentId>, on_undock: (), on_files_drop: Vec<String>, on_view_change: String, on_topmost_lost: (), on_scale_change: f64, on_resize_end: ();
        computed: client_width: f32 => client_width / set_client_width, client_height: f32 => client_height / set_client_height
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::{MethodInfo, PropKind};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("move_to_monitor", &[PropKind::Int], None).params(&["monitor"]),
            MethodInfo::new("request_attention", &[PropKind::Bool], None).params(&["critical"]),
            MethodInfo::new("bring_to_front", &[], None),
            MethodInfo::new("set_activation_token", &[PropKind::String], None).params(&["token"]),
            MethodInfo::new("window_scale", &[], Some(PropKind::Float)),
            MethodInfo::new("pointer_over", &[], Some(PropKind::Bool)),
            MethodInfo::new("place", &[PropKind::Int, PropKind::Int, PropKind::Int, PropKind::Int, PropKind::Int, PropKind::Bool], None)
                .params(&["monitor", "x", "y", "width", "height", "activate"]),
            MethodInfo::new("gpu_adapter", &[], Some(PropKind::String)),
            MethodInfo::new("gpu_adapters", &[], Some(PropKind::List)),
            // Onde caíram os arquivos do `on_files_drop` (nas coordenadas do form).
            MethodInfo::new("drop_x", &[], Some(PropKind::Float)),
            MethodInfo::new("drop_y", &[], Some(PropKind::Float)),
            // Atendido pelo `Ui::invoke` (o `Ui` pinta o form): os pixels RGBA em base64.
            MethodInfo::new("render_rgba", &[PropKind::Float, PropKind::Float, PropKind::Float, PropKind::Float, PropKind::Float], Some(PropKind::String))
                .params(&["x", "y", "width", "height", "scale"]),
        ];
        METHODS
    }

    /// `move_to_monitor(i)`: leva a janela para o monitor `i` (o índice do `Screen`).
    /// `request_attention(critical)`: chama a atenção para a janela sem tomar o foco (no
    /// Windows, o botão na barra de tarefas pisca até o usuário ativá-la; `critical` pisca a
    /// janela também).
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[aylis_core::PropValue]) -> Result<aylis_core::PropValue, String> {
        match method {
            "move_to_monitor" => {
                let index = args.first().and_then(|v| v.to_i64().ok()).filter(|i| *i >= 0).ok_or("move_to_monitor pede o índice do monitor")?;
                // O `monitor` acompanha: a tela cheia ligada depois vai para este monitor.
                self.monitor = index;
                ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::MoveToMonitor(index as usize));
                Ok(aylis_core::PropValue::Null)
            }
            // Lugar e tamanho em pixels físicos, a partir do canto do monitor.
            "place" => {
                let int = |i: usize| args.get(i).and_then(|v| v.to_i64().ok());
                let (Some(monitor), Some(x), Some(y), Some(width), Some(height)) = (int(0), int(1), int(2), int(3), int(4)) else {
                    return Err("place pede monitor, x, y, largura e altura".into());
                };
                if monitor < 0 || width <= 0 || height <= 0 {
                    return Err("place pede o monitor (0 ou mais) e largura e altura maiores que zero".into());
                }
                let activate = args.get(5).and_then(|v| v.to_bool().ok()).unwrap_or(false);
                let (x, y) = (x.clamp(i32::MIN as i64, i32::MAX as i64) as i32, y.clamp(i32::MIN as i64, i32::MAX as i64) as i32);
                let (width, height) = (width.min(u32::MAX as i64) as u32, height.min(u32::MAX as i64) as u32);
                ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::Place { monitor: monitor as usize, x, y, width, height, activate });
                Ok(aylis_core::PropValue::Null)
            }
            // Para a frente com o foco, mesmo sem a última entrada (no Wayland, com o token).
            "bring_to_front" => {
                ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::BringToFront);
                Ok(aylis_core::PropValue::Null)
            }
            // O token de ativação do Wayland que o próximo `bring_to_front` usa (o
            // `XDG_ACTIVATION_TOKEN` da outra abertura, entregue pelo app).
            "set_activation_token" => {
                let token = args.first().and_then(|v| v.to_str().ok()).unwrap_or_default();
                ctx.ui.set_activation_token(token);
                Ok(aylis_core::PropValue::Null)
            }
            // A escala real da janela (`Ui::form_scale`).
            "window_scale" => Ok(aylis_core::PropValue::Float(f64::from(ctx.ui.form_scale(ctx.id)))),
            // O mouse sobre a janela à vista (`Ui::pointer_over`; no Wayland, sempre `false`).
            "pointer_over" => Ok(aylis_core::PropValue::Bool(ctx.ui.pointer_over(ctx.id))),
            "request_attention" => {
                let critical = args.first().and_then(|v| v.to_bool().ok()).unwrap_or(false);
                ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::RequestAttention(critical));
                Ok(aylis_core::PropValue::Null)
            }
            // A placa em que a janela desenha agora (vazio: não se sabe, como na CPU).
            "gpu_adapter" => Ok(aylis_core::PropValue::String(ctx.ui.form_gpu(ctx.id).unwrap_or_default().to_string())),
            // As placas da máquina, separadas por `;` (o texto que a propriedade `gpu` aceita).
            "gpu_adapters" => Ok(aylis_core::PropValue::String(aylis_core::gpu::adapters().join(";"))),
            "drop_x" | "drop_y" => {
                let at = ctx.ui.files_drop_position(ctx.id).unwrap_or_default();
                Ok(aylis_core::PropValue::Float(f64::from(if method == "drop_x" { at.x } else { at.y })))
            }
            _ => Err(format!("Form não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "zoom" {
            ctx.ui.mark_zoom_changed();
        }
        // A orientação com o app aberto: o pedido é calculado na hora, e a plataforma aplica no
        // próximo quadro.
        if name == "orientation" {
            ctx.ui.request_repaint();
        }
        // Com o app rodando, a janela segue (no designer não há janela).
        if (name == "gpu" || name == "present") && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateGpu);
        }
        if name == "span_monitors" && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateSpan);
        }
        if name == "physical_pixels" && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateScale);
        }
        // `topmost` com a janela aberta: antes só valia na criação (RTSPlayer, 13/11, item 4).
        // O `host` mudado pelo código: entra ou sai, depois deste evento (mexe na árvore).
        if name == "host" && !ctx.ui.design_mode() && self.host != self.docked_in {
            let (form, host) = (ctx.id, self.host);
            ctx.ui.set_timeout(0, move |ui| {
                let done = match host {
                    Some(h) => crate::docking::dock_form(ui, form, h, None),
                    None => crate::docking::undock_form(ui, form, None),
                };
                if let Err(e) = done {
                    aylis_core::stderr_line!("aylis: o form não mudou de lugar ({e})");
                }
            });
        }
        if matches!(name, "topmost" | "bottommost") && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateLevel);
        }
        if name == "keep_awake" && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateKeepAwake);
        }
        if name == "confine_cursor" && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateCursorConfine);
        }
        if name == "window_opacity" && !ctx.ui.design_mode() {
            self.window_opacity = self.window_opacity.clamp(0.05, 1.0);
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateOpacity);
        }
        // O modo de vídeo mudado com o form em tela cheia: a tela cheia se refaz.
        if matches!(name, "full_screen_mode" | "video_width" | "video_height" | "video_refresh") && !ctx.ui.design_mode() && self.window_state == WindowState::FullScreen {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::SetFullscreen(true));
        }
        if matches!(name, "window_corners" | "border_style" | "window_shape" | "shape_radius" | "shape_radii" | "shape_path" | "transparent") && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateCorners);
        }
        // O tamanho mudado pelo código com o app rodando: a janela acompanha, com a área
        // cliente do tamanho pedido (a moldura fica por fora; J11).
        if matches!(name, "width" | "height" | "client_width" | "client_height") && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::ApplySize);
        }
        // O `monitor` mudado com o form aberto leva a janela para lá (em tela cheia, ela continua
        // em tela cheia no outro). Antes de abrir, só escolhe onde ele nasce.
        if name == "monitor" && !ctx.ui.design_mode() && self.monitor >= 0 && ctx.ui.is_open(ctx.id) && ctx.ui.form_monitor(ctx.id) != Some(self.monitor as usize) {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::MoveToMonitor(self.monitor as usize));
        }
        if name == "window_state" && !ctx.ui.design_mode() {
            let form = ctx.id;
            for command in window_state_commands(self.window_state) {
                ctx.ui.window_command(form, command);
            }
        }
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.active_control == Some(freed) {
            self.active_control = None;
        }
        if self.host == Some(freed) {
            self.host = None;
        }
        if self.docked_in == Some(freed) {
            self.docked_in = None;
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        // Com forma (`window_shape`), só dentro dela: fora, a janela transparente mostra o que
        // está atrás.
        let brush = Brush::Solid(ctx.style.fill);
        match self.clip_shape() {
            Some(shape) => shape.fill(canvas, ctx.rect, &brush),
            None => canvas.fill_rect(ctx.rect, 0.0, &brush),
        }
    }
}

/// Os pedidos à janela que levam a `state` (sai da tela cheia antes de maximizar e afins).
pub fn window_state_commands(state: WindowState) -> Vec<aylis_core::WindowCommand> {
    use aylis_core::WindowCommand as W;
    match state {
        WindowState::Normal => vec![W::SetFullscreen(false), W::Restore],
        WindowState::Maximized => vec![W::SetFullscreen(false), W::Maximize],
        WindowState::Minimized => vec![W::Minimize],
        WindowState::FullScreen => vec![W::SetFullscreen(true)],
    }
}

// ---------------------------------------------------------------------------

/// Contêiner invisível, só para organizar o layout.
pub struct Layout {
    pub base: ControlBase,
    /// Os caracteres dos campos dentro do layout (veja o `Form`).
    pub allowed_chars: String,
    /// Os caracteres que os campos de dentro não aceitam (digitados ou colados); vazio: nenhum.
    pub blocked_chars: String,
    /// Só os caracteres de `allowed_chars` entram nos campos de dentro.
    pub only_allowed: bool,
    /// O filtro de caracteres do layout manda nos campos de dentro (sem ele, o do campo decide
    /// primeiro).
    pub chars_override: bool,
    /// Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a
    /// mesma herança dos filtros.
    pub char_map: String,
    /// O vão entre os filhos encaixados (o `gap` do CSS).
    pub gap: f32,
}

impl Default for Layout {
    fn default() -> Self {
        let mut base = ControlBase::new(100.0, 100.0);
        base.set_hit_test(false);
        Self { base, allowed_chars: String::new(), blocked_chars: String::new(), only_allowed: false, chars_override: false, char_map: String::new(), gap: 0.0 }
    }
}

impl Component for Layout {
    fn layout_hooks(&self) -> Option<&dyn aylis_core::LayoutHooks> {
        Some(self)
    }

    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    leaf_control!("Layout");

    properties! {
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        chars_override: bool => "Keyboard",
        char_map: String => "Keyboard",
        gap: f32 => "Layout",
    }
}

impl aylis_core::LayoutHooks for Layout {
    fn dock_gap(&self) -> f32 {
        self.gap
    }
}

// ---------------------------------------------------------------------------

/// Um form do projeto dentro de outro: `form` é o nome dele.
/// Cada parte da tela fica no seu arquivo, desenhada e editada à parte, e o form que usa
/// só reserva o lugar. Ao carregar o projeto, a fábrica de forms cria o form do frame e o
/// põe aqui, ocupando a área toda (o fundo dele fica transparente, se ele não tiver um);
/// no designer, a IDE mostra o conteúdo sem deixar mexer nele. Os componentes do frame se
/// acham pelo nome do form dele (`frmConfig.edtPasta`) e os eventos vão para os handlers
/// desse form.
pub struct Frame {
    pub base: ControlBase,
    /// O nome do form do projeto mostrado aqui (`pkg:Nome` para o de um pacote); vazio: nada.
    pub form: String,
    /// Num frame de pacote (`form = "pkg:Nome"`): os valores das propriedades expostas pelo
    /// pacote, em JSON (`{"valor": 42}`).
    pub values: String,
}

impl Default for Frame {
    fn default() -> Self {
        Self { base: ControlBase::new(320.0, 200.0), form: String::new(), values: String::new() }
    }
}

impl Frame {
    pub fn new(form: impl Into<String>) -> Self {
        Self { form: form.into(), ..Default::default() }
    }
}

impl Component for Frame {
    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    leaf_control!("Frame");

    properties! {
        form: String as FormName => "Behavior",
        values: String => "Behavior",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        // No designer, um frame sem conteúdo (sem form, ou form que não existe) se mostra.
        if !ctx.design || !ctx.ui.children(ctx.id).is_empty() {
            return;
        }
        let color = Color { a: 140, ..ctx.style.text };
        canvas.stroke_rect(ctx.rect.inflate(-0.5), 0.0, &Stroke::new(color, 1.0).dashed(vec![4.0, 3.0]));
        let text = if self.form.is_empty() { "Frame (escolha o form)".to_string() } else { format!("Frame: {}", self.form) };
        let style = TextStyle { h_align: TextAlign::Center, v_align: TextAlign::Center, color, ..ctx.text_style() };
        canvas.draw_text(&text, ctx.rect, &style);
    }
}

// ---------------------------------------------------------------------------

/// Retângulo. Cada propriedade vazia (`Null`) usa o valor do tema.
#[derive(Default)]
pub struct Rectangle {
    pub base: ControlBase,
    pub fill: Option<Color>,
    /// Segunda cor: com ela, o fundo vira um gradiente de `fill` até `fill2`.
    pub fill2: Option<Color>,
    /// Cor do meio do gradiente (opcional), na posição `fill_mid_at`.
    pub fill_mid: Option<Color>,
    /// Onde fica a `fill_mid`, de 0 a 1 (0 = no meio).
    pub fill_mid_at: f32,
    /// Direção do gradiente em graus, como no CSS: 0 = para cima, 90 = para a direita.
    pub gradient_angle: f32,
    /// Gradiente radial: `fill` no centro até `fill2` na borda (o menor raio do retângulo).
    pub gradient_radial: bool,
    /// Cor da borda; vazio: a do tema.
    pub stroke: Option<Color>,
    /// Espessura da borda, em pontos (0 = sem borda); vazio: a do tema.
    pub stroke_width: Option<f32>,
    /// O tracejado da borda: traço e vão em pontos, alternados (`"6;4"`, `"1.5;3"`); `"0"` é
    /// contínua; vazio: o do tema (`border_style`).
    pub stroke_dash: String,
    /// Raio dos cantos: um número, ou `[cima-esquerda, cima-direita, baixo-direita, baixo-esquerda]`. Negativo curva o canto para fora (a aba que se funde no painel). Vazio: o do tema.
    pub radius: Option<Bounds>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self { base: ControlBase::new(width, height), ..Default::default() }
    }

    /// O `stroke_dash` lido: `None` sem ele (vale o tema); vazio para contínua.
    fn dash(&self) -> Option<Vec<f32>> {
        let text = self.stroke_dash.trim();
        if text.is_empty() {
            return None;
        }
        let values: Vec<f32> = text.split([';', ',', ' ']).filter_map(|v| v.trim().parse::<f32>().ok()).filter(|v| v.is_finite() && *v > 0.0).collect();
        // Um número só é traço e vão iguais; um número de valores ímpar se repete (como o CSS).
        Some(match values.len() {
            1 => vec![values[0], values[0]],
            n if n % 2 == 1 => values.iter().chain(values.iter()).copied().collect(),
            _ => values,
        })
    }
}

/// A borda do estilo em volta de `r`: por lado (`border_sides`, sem os cantos arredondados) ou
/// inteira, contínua ou tracejada (`border_style`). O relatório do Tabernáculo, C11 e C12.
pub fn stroke_style_border(canvas: &mut dyn Canvas, r: RectF, radius: f32, color: Color, width: f32, style: &aylis_core::ResolvedStyle) {
    if color.is_transparent() {
        return;
    }
    match style.border_sides {
        Some(sides) => {
            let brush = Brush::Solid(color);
            for (side, w) in [(0, sides.left), (1, sides.top), (2, sides.right), (3, sides.bottom)] {
                if w <= 0.0 {
                    continue;
                }
                if style.border_dash.is_empty() {
                    let line = match side {
                        0 => RectF::new(r.x, r.y, w, r.height),
                        1 => RectF::new(r.x, r.y, r.width, w),
                        2 => RectF::new(r.right() - w, r.y, w, r.height),
                        _ => RectF::new(r.x, r.bottom() - w, r.width, w),
                    };
                    canvas.fill_rect(line, 0.0, &brush);
                } else {
                    let half = w / 2.0;
                    let (a, b) = match side {
                        0 => (PointF::new(r.x + half, r.y), PointF::new(r.x + half, r.bottom())),
                        1 => (PointF::new(r.x, r.y + half), PointF::new(r.right(), r.y + half)),
                        2 => (PointF::new(r.right() - half, r.y), PointF::new(r.right() - half, r.bottom())),
                        _ => (PointF::new(r.x, r.bottom() - half), PointF::new(r.right(), r.bottom() - half)),
                    };
                    canvas.draw_line(a, b, &style.border_stroke(color, w));
                }
            }
        }
        None if width > 0.0 => canvas.stroke_rect(r, radius, &style.border_stroke(color, width)),
        None => {}
    }
}

/// Gradiente linear que atravessa `r` na direção `angle` (graus, como no CSS: 0 = de baixo
/// para cima, 90 = da esquerda para a direita), de canto a canto.
pub fn linear_gradient(r: RectF, angle: f32, from: Color, to: Color) -> Brush {
    linear_gradient_stops(r, angle, vec![GradientStop::new(0.0, from), GradientStop::new(1.0, to)])
}

/// O mesmo, com as paradas que quiser.
pub fn linear_gradient_stops(r: RectF, angle: f32, stops: Vec<GradientStop>) -> Brush {
    let a = angle.to_radians();
    let (dx, dy) = (a.sin(), -a.cos());
    let half = (r.width / 2.0 * dx).abs() + (r.height / 2.0 * dy).abs();
    let c = r.center();
    Brush::Linear {
        start: PointF::new(c.x - dx * half, c.y - dy * half),
        end: PointF::new(c.x + dx * half, c.y + dy * half),
        stops,
    }
}

impl Component for Rectangle {
    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    /// Cor sólida opaca sem cantos arredondados (o `Panel` herda): cobre o retângulo inteiro.
    fn opaque(&self, style: &aylis_core::ResolvedStyle) -> bool {
        let square = self.radius.map_or(style.radius <= 0.0, |c| [c.left, c.top, c.right, c.bottom].iter().all(|v| *v == 0.0));
        self.fill2.is_none() && self.fill.unwrap_or(style.fill).a == 255 && square
    }

    /// O `radius` próprio (um por canto, inclusive) para o Reveal acender pelos mesmos cantos
    /// que o fundo; sem ele, o raio do estilo.
    fn painted_shape(&self) -> Option<aylis_core::ClipShape> {
        self.radius.map(aylis_core::ClipShape::from_corners)
    }

    leaf_control!("Rectangle");

    properties! {
        fill: Option<Color> => "Appearance",
        fill2: Option<Color> => "Appearance",
        fill_mid: Option<Color> => "Appearance",
        fill_mid_at: f32 => "Appearance",
        gradient_angle: f32 as Angle => "Appearance",
        gradient_radial: bool => "Appearance",
        stroke: Option<Color> => "Appearance",
        stroke_width: Option<f32> => "Appearance",
        stroke_dash: String as NumberList => "Appearance",
        radius: Option<Bounds> => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        self.paint_filled(canvas, ctx, None);
    }
}

impl Rectangle {
    /// A pintura com outro fundo no lugar do `fill` (`None`: o de sempre): o `Panel` com o
    /// mouse em cima.
    pub fn paint_filled(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, fill_over: Option<Color>) {
        // O `stroke_dash` do próprio retângulo no lugar do traço do tema.
        let dashed;
        let style = match self.dash() {
            Some(dash) => {
                dashed = aylis_core::ResolvedStyle { border_dash: dash, ..ctx.style.clone() };
                &dashed
            }
            None => ctx.style,
        };
        let corners = self.radius.unwrap_or(aylis_core::Bounds::all(ctx.style.radius));
        let fill = fill_over.or(self.fill).unwrap_or(ctx.style.fill);
        // Um raio por canto (`radius: [8, 8, 0, 0]`, negativo curva para fora): pelo contorno.
        let uniform = corners.left == corners.top && corners.top == corners.right && corners.right == corners.bottom && corners.left >= 0.0;
        if !uniform {
            let path = aylis_core::Path::corners(ctx.rect, corners);
            let brush = match self.fill2 {
                Some(fill2) => linear_gradient(ctx.rect, self.gradient_angle, fill, fill2),
                None => Brush::Solid(fill),
            };
            if !fill.is_transparent() || self.fill2.is_some() {
                canvas.fill_path(&path, &brush);
            }
            let width = self.stroke_width.unwrap_or(ctx.style.border_width);
            let stroke = self.stroke.unwrap_or(ctx.style.border_color);
            if width > 0.0 && !stroke.is_transparent() {
                canvas.stroke_path(&path, &style.border_stroke(stroke, width));
            }
            return;
        }
        let radius = corners.left;
        if let Some(fill2) = self.fill2 {
            let mut stops = vec![GradientStop::new(0.0, fill)];
            if let Some(mid) = self.fill_mid {
                let at = if self.fill_mid_at > 0.0 { self.fill_mid_at.min(1.0) } else { 0.5 };
                stops.push(GradientStop::new(at, mid));
            }
            stops.push(GradientStop::new(1.0, fill2));
            let brush = if self.gradient_radial {
                Brush::Radial { center: ctx.rect.center(), radius: ctx.rect.width.min(ctx.rect.height) / 2.0, stops }
            } else {
                linear_gradient_stops(ctx.rect, self.gradient_angle, stops)
            };
            canvas.fill_rect(ctx.rect, radius, &brush);
        } else if !fill.is_transparent() {
            canvas.fill_rect(ctx.rect, radius, &Brush::Solid(fill));
        }
        let width = self.stroke_width.unwrap_or(ctx.style.border_width);
        let stroke = self.stroke.unwrap_or(ctx.style.border_color);
        stroke_style_border(canvas, ctx.rect, radius, stroke, width, style);
    }
}

// ---------------------------------------------------------------------------

/// Painel: um [`Rectangle`] com o estilo `Panel` do tema (fundo, borda e cantos).
///
/// É também o exemplo mais simples de "herança": todo o comportamento e as
/// propriedades vêm do `Rectangle`; só o nome da classe (e portanto o estilo) muda.
#[derive(Default)]
pub struct Panel {
    pub base: Rectangle,
    /// Os caracteres dos campos dentro do painel (veja o `Form`).
    pub allowed_chars: String,
    /// Os caracteres que os campos de dentro não aceitam (digitados ou colados); vazio: nenhum.
    pub blocked_chars: String,
    /// Só os caracteres de `allowed_chars` entram nos campos de dentro.
    pub only_allowed: bool,
    /// O filtro de caracteres do painel manda nos campos de dentro (sem ele, o do campo decide
    /// primeiro).
    pub chars_override: bool,
    /// Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a
    /// mesma herança dos filtros.
    pub char_map: String,
    /// O vão entre os filhos encaixados (o `gap` do CSS; L05 do Tabernáculo).
    pub gap: f32,
    /// O fundo com o mouse em cima (nele ou num filho); vazio: como sem mouse.
    pub hover_fill: Option<Color>,
    /// O fundo apertado; vazio: o `hover_fill`.
    pub pressed_fill: Option<Color>,
    /// Reage ao mouse com as cores de estado do tema (o mesmo clarear dos botões), sem
    /// escolher cor: a linha de uma lista, um "botão" feito de painel (decidido em 21/11). O
    /// `hover_fill` e o `pressed_fill`, quando dados, mandam.
    pub interactive: bool,
    /// O fundo marcado (a linha escolhida de uma lista, o estado `checked`); vazio: com
    /// `interactive`, a cor principal do tema, clara.
    pub selected_fill: Option<Color>,
}

impl Panel {
    pub fn new(width: f32, height: f32) -> Self {
        Self { base: Rectangle::new(width, height), ..Default::default() }
    }

    /// Reage ao mouse (`interactive`, `hover_fill` ou `pressed_fill`).
    fn reacts(&self) -> bool {
        self.interactive || self.hover_fill.is_some() || self.pressed_fill.is_some() || self.selected_fill.is_some()
    }
}

impl Component for Panel {
    fn layout_hooks(&self) -> Option<&dyn aylis_core::LayoutHooks> {
        Some(self)
    }

    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    fn class_name(&self) -> &'static str {
        "Panel"
    }

    properties! {
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        chars_override: bool => "Keyboard",
        char_map: String => "Keyboard",
        gap: f32 => "Layout",
        hover_fill: Option<Color> => "Appearance",
        pressed_fill: Option<Color> => "Appearance",
        interactive: bool => "Behavior",
        selected_fill: Option<Color> => "Appearance",
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if matches!(name, "hover_fill" | "pressed_fill" | "interactive" | "selected_fill") {
            let on = self.reacts();
            self.base.base.set_hover_within(on);
            if on && self.base.base.cursor() == aylis_core::CursorShape::Default && self.interactive {
                self.base.base.set_cursor(aylis_core::CursorShape::Pointer);
            }
            ctx.repaint();
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        if !self.reacts() || ctx.design {
            return self.base.paint(canvas, ctx);
        }
        let (hover, pressed) = ctx.ui.hover_within(ctx.id);
        let (hover, pressed) = (hover || ctx.state.hover, pressed || ctx.state.pressed);
        let rest = self.base.fill.unwrap_or(ctx.style.fill);
        // Marcado (a linha escolhida): o fundo de marcado por baixo do mouse.
        let rest = if ctx.state.checked {
            self.selected_fill.unwrap_or_else(|| if self.interactive { rest.lerp(ctx.theme.color("primary"), 0.22) } else { rest })
        } else {
            rest
        };
        let fill = if pressed {
            self.pressed_fill.or(self.hover_fill)
        } else if hover {
            self.hover_fill
        } else {
            None
        };
        // Sem cor própria: as do tema (o `hover` do tema, o clarear dos botões); marcado sem
        // mouse, o fundo de marcado.
        let fill = fill.or_else(|| {
            (self.interactive && (hover || pressed)).then(|| {
                let toward = ctx.theme.color("text");
                let base = if rest.is_transparent() { toward.with_opacity(0.0) } else { rest };
                base.lerp(toward, if pressed { 0.16 } else { 0.08 })
            })
        });
        let fill = fill.or(ctx.state.checked.then_some(rest));
        self.base.paint_filled(canvas, ctx, fill);
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }
}

impl aylis_core::LayoutHooks for Panel {
    fn dock_gap(&self) -> f32 {
        self.gap
    }
}

// ---------------------------------------------------------------------------

/// Texto.
pub struct Label {
    pub base: ControlBase,
    /// O texto mostrado.
    pub text: String,
    /// Alinhamento horizontal do texto: começo, centro ou fim.
    pub h_align: TextAlign,
    /// Alinhamento vertical do texto: em cima, no meio ou embaixo.
    pub v_align: TextAlign,
    /// Quebra automática de linha.
    pub wrap: bool,
    /// Reticências quando não couber: por caractere, por palavra ou nenhuma.
    pub trimming: Trimming,
    /// Negrito (atalho do `font_weight` = `bold`; a fonte em si é a do `ControlBase`:
    /// `font_family`, `font_size`...).
    pub bold: bool,
    /// Cor do texto; `None` usa a do tema.
    pub color: Option<Color>,
    /// Trecho a destacar (o `v-highlight`): cada ocorrência, sem diferenciar maiúsculas e
    /// acentos, ganha o fundo marcado (cor `highlight` do tema, ou `warning` clara).
    pub highlight: String,
    /// O campo que recebe o foco no Alt+letra do `&` do texto.
    pub focus_control: Option<ComponentId>,
    /// O texto traz marcas simples no meio, que viram estilo em vez de aparecer (os textos com
    /// HTML simples de um app web): `<b>` negrito, `<sb>` seminegrito, `<i>` itálico, `<u>`
    /// sublinhado, `<code>` fonte mono, `<small>` e `<big>` (85% e 120%), `<size=18>`,
    /// `<color=text_muted>` (token do tema ou `#rrggbb`), `<br>` (quebra) e
    /// `<tag=fundo,borda>` (a etiqueta: fundo e borda, cada um token ou `#rrggbb`, a borda
    /// opcional; o raio `sm` e o espaçamento `xs` do tema; não quebra no meio). Ver
    /// [`markup_spans`].
    pub markup: bool,
    /// Contorno em volta das letras (a legenda sobre vídeo: legível em qualquer fundo).
    pub outline: Option<Color>,
    /// Espessura do contorno, em pixels (0 = sem contorno).
    pub outline_width: f32,
    /// Espaço a mais entre as letras, em pixels (os títulos em caixa-alta espaçados).
    pub letter_spacing: f32,
    /// Maiúsculas ou minúsculas ao desenhar (o texto não muda).
    pub text_case: TextCase,
    /// A altura de cada linha do texto quebrado, em vezes o tamanho da fonte (o `line-height`
    /// do CSS); 0: a do estilo (`line_height`) ou a da fonte. O relatório do Tabernáculo, T04.
    pub line_height: f32,
    /// Sombra das letras (a legenda sobre vídeo): a cor, o desfoque e o deslocamento (T02).
    pub text_shadow: Option<aylis_core::Shadow>,
    /// Com `wrap`, no máximo estas linhas, com o `trimming` na última (o `line-clamp` do CSS);
    /// 0: as que couberem na altura.
    pub max_lines: u32,
}

impl Default for Label {
    fn default() -> Self {
        let mut base = ControlBase::new(120.0, 20.0);
        base.set_hit_test(false);
        Self {
            base,
            text: String::new(),
            h_align: TextAlign::Leading,
            v_align: TextAlign::Center,
            wrap: false,
            trimming: Trimming::Character,
            highlight: String::new(),
            bold: false,
            color: None,
            focus_control: None,
            letter_spacing: 0.0,
            text_case: TextCase::None,
            markup: false,
            outline: None,
            outline_width: 0.0,
            line_height: 0.0,
            text_shadow: None,
            max_lines: 0,
        }
    }
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    /// A altura de linha em vigor: a do controle ou a do estilo (0: a da fonte).
    fn line_height_of(&self, style: f32) -> f32 {
        if self.line_height > 0.0 { self.line_height } else { style }
    }

    /// O texto quebrado linha a linha, com a altura de linha `line_height` (em vezes a fonte;
    /// 0: a da fonte). As linhas param na altura da área e em `max_lines` (0: sem limite), e o
    /// `trimming` entra na última (antes descia além da altura, por cima dos vizinhos; o
    /// relatório do player de música, 21/11).
    /// `highlight`: a busca e a cor do fundo marcado em cada ocorrência (o `highlight` do
    /// Label, que este caminho não desenhava).
    fn paint_lines(canvas: &mut dyn Canvas, text: &str, area: RectF, style: &TextStyle, line_height: f32, max_lines: u32, highlight: Option<(&str, Color)>) {
        let mut one = style.clone();
        one.wrap = false;
        one.v_align = TextAlign::Center;
        let step = if line_height > 0.0 { style.font.size * line_height } else { canvas.measure_text("Ag", &one).height.max(1.0) };
        let fit = ((area.height + 0.5) / step).floor().max(1.0) as usize;
        let limit = if max_lines > 0 { fit.min(max_lines as usize) } else { fit };
        let mut measure = |s: &str| canvas.measure_text(s, &one).width;
        let lines = aylis_core::text::layout_lines(text, area.width, limit, true, style.trimming, &mut measure);
        let total = step * lines.len() as f32;
        let mut y = match style.v_align {
            TextAlign::Leading | TextAlign::Justify => area.y,
            TextAlign::Center => area.y + ((area.height - total) / 2.0).max(0.0),
            TextAlign::Trailing => area.y + (area.height - total).max(0.0),
        };
        for line in &lines {
            if let Some((query, mark)) = highlight {
                let width = canvas.measure_text(line, &one).width;
                let x = match style.h_align {
                    TextAlign::Leading | TextAlign::Justify => area.x,
                    TextAlign::Center => area.x + (area.width - width) / 2.0,
                    TextAlign::Trailing => area.right() - width,
                };
                for (a, b) in find_matches(line, query) {
                    let prefix: String = line.chars().take(a).collect();
                    let part: String = line.chars().skip(a).take(b - a).collect();
                    let x0 = x + canvas.measure_text(&prefix, &one).width;
                    let w = canvas.measure_text(&part, &one).width;
                    canvas.fill_rect(RectF::new(x0 - 1.0, y + 1.0, w + 2.0, step - 2.0), 3.0, &Brush::Solid(mark));
                }
            }
            canvas.draw_text(line, RectF::new(area.x, y, area.width, step), &one);
            y += step;
        }
    }

    /// A sombra das letras: cópias deslocadas na cor da sombra, mais fracas em volta para o
    /// desfoque (o mesmo nos três renderizadores, sem efeito próprio de nenhum).
    fn paint_text_shadow(canvas: &mut dyn Canvas, text: &str, area: RectF, style: &TextStyle, shadow: &aylis_core::Shadow) {
        let mut s = style.clone();
        let at = area.offset(shadow.offset.x, shadow.offset.y);
        if shadow.blur <= 0.5 {
            s.color = shadow.color;
            canvas.draw_text(text, at, &s);
            return;
        }
        let ring = shadow.blur * 0.35;
        s.color = shadow.color.with_opacity(0.22);
        for k in 0..8 {
            let a = k as f32 * std::f32::consts::TAU / 8.0;
            canvas.draw_text(text, at.offset(a.cos() * ring, a.sin() * ring), &s);
        }
        s.color = shadow.color.with_opacity(0.45);
        canvas.draw_text(text, at, &s);
    }
}

impl Component for Label {
    fn layout_hooks(&self) -> Option<&dyn aylis_core::LayoutHooks> {
        Some(self)
    }

    leaf_control!("Label");

    // Sem fundo: o `fill` próprio vira o fundo do controle.
    fn transparent_background(&self) -> bool {
        true
    }

    properties! {
        text: String => "Appearance",
        h_align: TextAlign => "Appearance",
        v_align: TextAlign => "Appearance",
        wrap: bool => "Appearance",
        trimming: Trimming => "Appearance",
        bold: bool => "Appearance",
        color: Option<Color> => "Appearance",
        highlight: String => "Appearance",
        letter_spacing: f32 => "Appearance",
        text_case: TextCase => "Appearance",
        markup: bool => "Appearance",
        outline: Option<Color> => "Appearance",
        outline_width: f32 => "Appearance",
        line_height: f32 => "Appearance",
        text_shadow: Option<aylis_core::Shadow> => "Appearance",
        max_lines: u32 => "Appearance",
        focus_control: Option<ComponentId> => "Behavior",
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::{MethodInfo, PropKind};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("min_content_width", &[], Some(PropKind::Float)),
            MethodInfo::new("max_content_width", &[], Some(PropKind::Float)),
        ];
        METHODS
    }

    /// `min_content_width()`: a palavra mais larga; `max_content_width()`: o texto numa linha
    /// só (o `min-content` e o `max-content` do CSS), com o padding (T02 do Tabernáculo).
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, _args: &[aylis_core::PropValue]) -> Result<aylis_core::PropValue, String> {
        let (ui, id) = (&*ctx.ui, ctx.id);
        let mut font = ui.font_of(id);
        if self.bold {
            font.weight = aylis_core::FontWeight::Bold;
        }
        let style = TextStyle { font, ..Default::default() };
        let text = if self.markup { markup_plain(&self.text) } else { self.text_case.apply(&self.text).to_string() };
        let pad = add_bounds(ui.style_padding(id), self.base.padding()).horizontal();
        let width = match method {
            "min_content_width" => text.split_whitespace().map(|w| ui.measure_text(w, &style).width).fold(0.0f32, f32::max),
            "max_content_width" => text.lines().map(|l| ui.measure_text(l, &style).width).fold(0.0f32, f32::max),
            _ => return Err(format!("método desconhecido: {method}")),
        };
        Ok(aylis_core::PropValue::Float(f64::from((width + pad).ceil())))
    }

    fn mnemonic_text(&self) -> Option<&str> {
        Some(&self.text)
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.focus_control == Some(freed) {
            self.focus_control = None;
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let mut style = ctx.text_style();
        style.h_align = self.h_align;
        style.v_align = self.v_align;
        style.wrap = self.wrap;
        style.trimming = self.trimming;
        if self.bold {
            style.font.weight = aylis_core::FontWeight::Bold;
        }
        if let Some(color) = self.color {
            style.color = color;
        }
        // O fundo, a borda e o raio do estilo do tema (o `fill` próprio do controle já foi
        // pintado pelo núcleo; o relatório do Tabernáculo, C10).
        if ctx.colors.fill.is_none() && !ctx.style.fill.is_transparent() {
            canvas.fill_rect(ctx.rect, ctx.style.radius, &Brush::Solid(ctx.style.fill));
        }
        stroke_style_border(canvas, ctx.rect, ctx.style.radius, ctx.style.border_color, ctx.style.border_width, ctx.style);
        // O `padding` do controle e o do estilo (antes só o do estilo; o relatório do
        // Tabernáculo, C05).
        let area = ctx.rect.deflate(add_bounds(ctx.style.padding, self.base.padding()));
        // A caixa-alta e o espaçamento do tema, quando o controle não diz os dele (C09).
        let text_case = if self.text_case == aylis_core::TextCase::None { ctx.style.text_case } else { self.text_case };
        let letter_spacing = if self.letter_spacing == 0.0 { ctx.style.letter_spacing } else { self.letter_spacing };
        if let Some(color) = self.outline.filter(|_| self.outline_width > 0.0 && !self.markup) {
            // O texto repetido em volta, na cor do contorno, antes do texto em si.
            let mut halo = style.clone();
            halo.color = color;
            let (text, _) = crate::mnemonic::caption(&text_case.apply(&self.text));
            let w = self.outline_width;
            for ring in [w, w * 0.5] {
                for k in 0..16 {
                    let a = k as f32 * std::f32::consts::TAU / 16.0;
                    canvas.draw_text(&text, area.offset(a.cos() * ring, a.sin() * ring), &halo);
                }
            }
        }
        let line_height = self.line_height_of(ctx.style.line_height);
        if let Some(shadow) = self.text_shadow.as_ref().filter(|_| !self.markup) {
            let (text, _) = crate::mnemonic::caption(&text_case.apply(&self.text));
            Self::paint_text_shadow(canvas, &text, area, &style, shadow);
        }
        if (line_height > 0.0 || self.max_lines > 0) && self.wrap && !self.markup && letter_spacing == 0.0 {
            let (text, _) = crate::mnemonic::caption(&text_case.apply(&self.text));
            let query = self.highlight.trim();
            let highlight = (!query.is_empty()).then(|| (query, highlight_color(ctx.theme)));
            Self::paint_lines(canvas, &text, area, &style, line_height, self.max_lines, highlight);
        } else if self.markup {
            paint_runs(canvas, markup_spans(&self.text), area, &style, letter_spacing, ctx.theme);
        } else if letter_spacing != 0.0 && self.wrap {
            // Com quebra de linha: as palavras em linhas, cada linha com o espaçamento (antes o
            // espaçamento era ignorado com `wrap` ou `markup`; o relatório do Tabernáculo, C04).
            let (text, _) = crate::mnemonic::caption(&self.text);
            let text = text_case.apply(&text);
            let runs = text.split('\n').enumerate().flat_map(|(i, line)| {
                let br = (i > 0).then(|| MarkupSpan::plain("\n"));
                br.into_iter().chain(std::iter::once(MarkupSpan::plain(line)))
            });
            paint_runs(canvas, runs.collect(), area, &style, letter_spacing, ctx.theme);
        } else if letter_spacing != 0.0 {
            let (text, _) = crate::mnemonic::caption(&self.text);
            draw_spaced(canvas, &text_case.apply(&text), area, &style, letter_spacing);
        } else if self.highlight.trim().is_empty() {
            let (text, at) = crate::mnemonic::caption(&text_case.apply(&self.text));
            crate::mnemonic::draw_text(canvas, &text, at, area, &style);
        } else {
            paint_highlighted(canvas, ctx, &self.text, &self.highlight, area, &style);
        }
    }
}

impl aylis_core::LayoutHooks for Label {
    fn baseline(&self, ui: &aylis_core::Ui, id: ComponentId, height: f32) -> Option<f32> {
        let pad = add_bounds(ui.style_padding(id), self.base.padding());
        Some(text_baseline(ui, id, height, pad, self.v_align))
    }

    /// `auto_size`: a altura do texto (quebrado na largura, com `wrap`), mais o padding do
    /// estilo. Sem texto, zero.
    fn auto_height(&self, ui: &aylis_core::Ui, id: ComponentId, width: f32) -> Option<f32> {
        let mut font = ui.font_of(id);
        if self.bold {
            font.weight = aylis_core::FontWeight::Bold;
        }
        let text = if self.markup { markup_plain(&self.text) } else { self.text_case.apply(&self.text).to_string() };
        if text.is_empty() {
            return Some(0.0);
        }
        let pad = add_bounds(ui.style_padding(id), self.base.padding());
        let line_height = self.line_height_of(ui.style_line_height(id));
        let style = TextStyle { font, wrap: self.wrap, ..Default::default() };
        // O negrito do markup é um pouco mais largo que o texto puro: mede com folga.
        let inner = (width - pad.horizontal()).max(1.0) * if self.markup { 0.97 } else { 1.0 };
        // Com a altura de linha própria ou o `max_lines`: as linhas quebradas (até o limite)
        // vezes a altura de cada uma.
        if (line_height > 0.0 || self.max_lines > 0) && self.wrap && !self.markup {
            let mut measure = |s: &str| ui.measure_text(s, &style).width;
            let limit = if self.max_lines > 0 { self.max_lines as usize } else { usize::MAX };
            let lines = aylis_core::text::layout_lines(&text, inner, limit, true, Trimming::None, &mut measure).len().max(1);
            let mut one = style.clone();
            one.wrap = false;
            let step = if line_height > 0.0 { style.font.size * line_height } else { ui.measure_text("Ag", &one).height.max(1.0) };
            return Some((lines as f32 * step).ceil() + pad.vertical() + 1.0);
        }
        let size = if self.wrap { ui.measure_text_wrapped(&text, &style, inner) } else { ui.measure_text(&text, &style) };
        Some(size.height + pad.vertical() + 1.0)
    }

    /// `auto_size` sem quebra de linha: a largura do texto mais o padding (com `align` left ou
    /// right o rótulo encolhe ao texto; relatório do InsightDB, 11/11).
    fn auto_width(&self, ui: &aylis_core::Ui, id: ComponentId) -> Option<f32> {
        // Solto (`align = none`), a largura desenhada vale (o texto centrado numa largura fixa).
        if self.wrap || ui.control(id).is_some_and(|cb| cb.align() == aylis_core::Align::None) {
            return None;
        }
        let mut font = ui.font_of(id);
        if self.bold {
            font.weight = aylis_core::FontWeight::Bold;
        }
        let text = if self.markup { markup_plain(&self.text) } else { self.text_case.apply(&self.text).to_string() };
        let pad = add_bounds(ui.style_padding(id), self.base.padding());
        let style = TextStyle { font, ..Default::default() };
        // O negrito do markup é um pouco mais largo que o texto puro: mede com folga.
        let tags = if self.markup { markup_tag_room(&self.text, ui.theme_of(id)) } else { 0.0 };
        Some((ui.measure_text(&text, &style).width * if self.markup { 1.04 } else { 1.0 } + tags + pad.horizontal() + 2.0).ceil())
    }
}

/// Um trecho do texto com marcas do Label (`markup`), com o estilo dele.
#[derive(Clone, Debug, PartialEq)]
pub struct MarkupSpan {
    /// O texto; `"\n"` sozinho é a quebra de linha.
    pub text: String,
    /// O peso (`<b>`: negrito; `<sb>`: seminegrito); `None`: o da fonte.
    pub weight: Option<aylis_core::FontWeight>,
    pub italic: bool,
    pub underline: bool,
    pub mono: bool,
    /// A cor (`<color=...>`): token do tema ou `#rrggbb`.
    pub color: Option<String>,
    /// O tamanho: `<size=N>` em pixels...
    pub size: Option<f32>,
    /// ...ou proporcional à fonte (`<small>` 0,85, `<big>` 1,2).
    pub scale: f32,
    /// A etiqueta em volta (`<tag=fundo,borda>`): os trechos da mesma etiqueta têm o mesmo
    /// [`MarkupTag::id`] e são desenhados juntos, sem quebrar.
    pub tag: Option<MarkupTag>,
}

/// A etiqueta de um trecho (`<tag=fundo,borda>`).
#[derive(Clone, Debug, PartialEq)]
pub struct MarkupTag {
    /// O fundo: token do tema ou `#rrggbb`.
    pub fill: String,
    /// A borda (`None`: sem borda).
    pub border: Option<String>,
    /// Qual etiqueta do texto (a ordem em que abriu), para juntar os trechos dela.
    pub id: usize,
}

impl MarkupSpan {
    fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), weight: None, italic: false, underline: false, mono: false, color: None, size: None, scale: 1.0, tag: None }
    }
}

/// Os trechos do texto com marcas (veja o `markup` do [`Label`]). As marcas se aninham; uma
/// que fecha sem ter aberto é ignorada, e o texto entre `<` e `>` que não é marca fica como
/// está. O relatório do Tabernáculo, C23.
pub fn markup_spans(text: &str) -> Vec<MarkupSpan> {
    let mut spans = Vec::new();
    // A pilha das marcas abertas: cada uma com o estilo de antes dela.
    let mut stack: Vec<(String, MarkupSpan)> = Vec::new();
    let mut cur = MarkupSpan::plain("");
    let mut rest = text;
    let mut buf = String::new();
    let mut tags = 0;
    let flush = |buf: &mut String, cur: &MarkupSpan, spans: &mut Vec<MarkupSpan>| {
        if !buf.is_empty() {
            spans.push(MarkupSpan { text: std::mem::take(buf), ..cur.clone() });
        }
    };
    while let Some(lt) = rest.find('<') {
        buf.push_str(&rest[..lt]);
        rest = &rest[lt..];
        let Some(gt) = rest.find('>') else { break };
        let tag = &rest[1..gt];
        let (closing, body) = match tag.strip_prefix('/') {
            Some(b) => (true, b),
            None => (false, tag.trim_end_matches('/')),
        };
        let (name, arg) = match body.split_once('=') {
            Some((n, a)) => (n.trim(), Some(a.trim().trim_matches('"'))),
            None => (body.trim(), None),
        };
        let known = matches!(name, "b" | "sb" | "i" | "u" | "code" | "small" | "big" | "size" | "color" | "br" | "tag");
        if !known {
            buf.push_str(&rest[..=gt]);
            rest = &rest[gt + 1..];
            continue;
        }
        flush(&mut buf, &cur, &mut spans);
        rest = &rest[gt + 1..];
        if name == "br" {
            spans.push(MarkupSpan { text: "\n".into(), ..cur.clone() });
            continue;
        }
        if closing {
            if let Some(pos) = stack.iter().rposition(|(n, _)| n == name) {
                cur = stack[pos].1.clone();
                stack.truncate(pos);
            }
            continue;
        }
        stack.push((name.to_string(), cur.clone()));
        match name {
            "b" => cur.weight = Some(aylis_core::FontWeight::Bold),
            "sb" => cur.weight = Some(aylis_core::FontWeight::SemiBold),
            "i" => cur.italic = true,
            "u" => cur.underline = true,
            "code" => cur.mono = true,
            "small" => cur.scale *= 0.85,
            "big" => cur.scale *= 1.2,
            "size" => cur.size = arg.and_then(|a| a.parse().ok()).or(cur.size),
            "tag" => {
                // `<tag=fundo,borda>`; sem cor, o fundo marcado do tema. Etiqueta dentro de
                // etiqueta fica na de fora (a de dentro não abre outra).
                if cur.tag.is_none() {
                    let (fill, border) = match arg.map(|a| a.split_once(',').map_or((a, None), |(f, b)| (f, Some(b)))) {
                        Some((fill, border)) => (fill.trim(), border.map(str::trim).filter(|b| !b.is_empty())),
                        None => ("", None),
                    };
                    let fill = if fill.is_empty() { "surface_2" } else { fill };
                    cur.tag = Some(MarkupTag { fill: fill.to_string(), border: border.map(str::to_string), id: tags });
                    tags += 1;
                }
            }
            _ => cur.color = arg.map(str::to_string).or(cur.color.take()),
        }
    }
    buf.push_str(rest);
    flush(&mut buf, &cur, &mut spans);
    spans
}

/// Os trechos do texto com marcas: (texto, negrito, mono). A forma curta do [`markup_spans`].
pub fn markup_runs(text: &str) -> Vec<(String, bool, bool)> {
    markup_spans(text).into_iter().map(|s| (s.text, s.weight == Some(aylis_core::FontWeight::Bold), s.mono)).collect()
}

/// O texto sem as marcas (para medir).
pub fn markup_plain(text: &str) -> String {
    markup_spans(text).into_iter().map(|s| s.text).collect()
}

/// Texto com negrito/mono no meio, quebrando nas palavras na largura da área.
/// A linha de base de uma linha de texto na fonte de `id`, alinhada por `v_align` na altura
/// `height` com o recuo `pad`: o topo da linha mais a parte de cima das letras (a subida, cerca
/// de 80% da altura da linha nas fontes de interface).
pub fn text_baseline(ui: &aylis_core::Ui, id: ComponentId, height: f32, pad: aylis_core::Bounds, v_align: TextAlign) -> f32 {
    let style = TextStyle { font: ui.font_of(id), ..Default::default() };
    let line = ui.measure_text("Ág", &style).height;
    let inner = (height - pad.vertical()).max(0.0);
    let top = match v_align {
        TextAlign::Leading | TextAlign::Justify => pad.top,
        TextAlign::Center => pad.top + (inner - line) / 2.0,
        TextAlign::Trailing => pad.top + inner - line,
    };
    top + line * 0.8
}

/// A soma de dois recuos.
fn add_bounds(a: aylis_core::Bounds, b: aylis_core::Bounds) -> aylis_core::Bounds {
    aylis_core::Bounds { left: a.left + b.left, top: a.top + b.top, right: a.right + b.right, bottom: a.bottom + b.bottom }
}

/// A largura de uma palavra com `spacing` pixels entre as letras.
fn spaced_width(canvas: &mut dyn Canvas, word: &str, style: &TextStyle, spacing: f32) -> f32 {
    if spacing == 0.0 {
        return canvas.measure_text(word, style).width;
    }
    let n = word.chars().count();
    word.chars().map(|c| canvas.measure_text(&c.to_string(), style).width).sum::<f32>() + spacing * n.saturating_sub(1) as f32
}

/// Desenha os trechos quebrando em palavras na largura de `area`; `"\n"` sozinho é quebra
/// de linha. Cada linha tem a altura do trecho mais alto dela, e as palavras ficam na mesma
/// base (o `<big>` no meio de uma frase não sobe o resto).
fn paint_runs(canvas: &mut dyn Canvas, runs: Vec<MarkupSpan>, area: RectF, style: &TextStyle, spacing: f32, theme: &aylis_core::Theme) {
    let mut one = style.clone();
    one.h_align = TextAlign::Leading;
    one.v_align = TextAlign::Leading;
    one.wrap = false;
    let base_h = canvas.measure_text("Ág", &one).height.max(1.0);
    let (mut x, mut line) = (area.x, 0usize);
    // A altura de cada linha (a do trecho mais alto).
    let mut heights: Vec<f32> = vec![base_h];
    // Primeiro as posições (linha e x); depois o desenho, com a altura de cada linha e o
    // alinhamento vertical.
    let mut placed: Vec<(String, TextStyle, f32, usize, f32, f32)> = Vec::new();
    // As etiquetas: o fundo, a borda, a linha e o x de onde começam e a largura, desenhadas
    // antes do texto.
    let mut boxes: Vec<(Color, Option<Color>, usize, f32, f32, f32)> = Vec::new();
    let pad = theme.spacing.get("xs").copied().unwrap_or(4.0);
    let tag_radius = theme.radius.get("sm").copied().unwrap_or(4.0);
    let color_of = |name: &str| theme.colors.get(name).copied().or_else(|| theme.resolve_color(name));
    let mut runs = runs.into_iter().peekable();
    while let Some(span) = runs.next() {
        if let Some(tag) = span.tag.clone() {
            // A etiqueta inteira: os trechos dela, sem quebrar, numa caixa só.
            let mut parts = vec![span];
            while runs.peek().is_some_and(|s| s.tag.as_ref().is_some_and(|t| t.id == tag.id) && s.text != "\n") {
                parts.extend(runs.next());
            }
            let styled: Vec<(String, TextStyle)> = parts.into_iter().map(|p| {
                let st = markup_style(&one, &p, theme);
                (p.text, st)
            }).collect();
            let widths: Vec<f32> = styled.iter().map(|(t, st)| spaced_width(canvas, t, st, spacing)).collect();
            let h = styled.iter().map(|(_, st)| if st.font.size == one.font.size { base_h } else { canvas.measure_text("Ág", st).height.max(1.0) }).fold(base_h, f32::max);
            let total = widths.iter().sum::<f32>() + pad * 2.0;
            if x > area.x {
                // O espaço antes da etiqueta é o do texto de fora (já somado); ela não cabe: desce.
                if x + total > area.right() + 0.5 {
                    x = area.x;
                    line += 1;
                    heights.push(base_h);
                }
            }
            heights[line] = heights[line].max(h);
            let fill = color_of(&tag.fill).unwrap_or(Color::TRANSPARENT);
            let border = tag.border.as_deref().and_then(color_of);
            boxes.push((fill, border, line, x, total, h));
            let mut tx = x + pad;
            for ((text, st), w) in styled.into_iter().zip(widths) {
                placed.push((text, st, tx, line, w, h));
                tx += w;
            }
            x += total;
            continue;
        }
        let st = markup_style(&one, &span, theme);
        if span.text == "\n" {
            x = area.x;
            line += 1;
            heights.push(base_h);
            continue;
        }
        let run = span.text;
        let h = if st.font.size == one.font.size { base_h } else { canvas.measure_text("Ág", &st).height.max(1.0) };
        let space = (canvas.measure_text("a a", &st).width - canvas.measure_text("aa", &st).width).max(1.0);
        // Palavras com o espaço que vem depois.
        let mut first = true;
        for word in run.split(' ') {
            if !first {
                x += space;
            }
            first = false;
            if word.is_empty() {
                continue;
            }
            let w = spaced_width(canvas, word, &st, spacing);
            if x + w > area.right() + 0.5 && x > area.x {
                x = area.x;
                line += 1;
                heights.push(base_h);
            }
            heights[line] = heights[line].max(h);
            placed.push((word.to_string(), st.clone(), x, line, w, h));
            x += w;
        }
    }
    let total: f32 = if placed.is_empty() { 0.0 } else { heights.iter().sum() };
    let dy = match style.v_align {
        TextAlign::Leading | TextAlign::Justify => 0.0,
        TextAlign::Center => ((area.height - total) / 2.0).max(0.0),
        TextAlign::Trailing => (area.height - total).max(0.0),
    };
    // O topo de cada linha.
    let tops: Vec<f32> = heights.iter().scan(area.y + dy, |y, h| {
        let top = *y;
        *y += h;
        Some(top)
    }).collect();
    for (fill, border, line, x, w, h) in boxes {
        let r = RectF::new(x, tops[line] + (heights[line] - h) * 0.8, w, h);
        canvas.fill_rect(r, tag_radius, &Brush::Solid(fill));
        if let Some(border) = border {
            canvas.stroke_rect(r.inflate(-0.5), tag_radius, &aylis_core::Stroke::new(border, 1.0));
        }
    }
    for (word, st, x, line, w, h) in placed {
        // Na base da linha: o trecho menor desce o que falta da altura dela.
        let y = tops[line] + (heights[line] - h) * 0.8;
        if spacing == 0.0 {
            canvas.draw_text(&word, RectF::new(x, y, w + 2.0, h), &st);
        } else {
            draw_spaced(canvas, &word, RectF::new(x, y, w + 2.0, h), &st, spacing);
        }
    }
}

/// O estilo de um trecho com marcas sobre o estilo `one` do texto.
fn markup_style(one: &TextStyle, span: &MarkupSpan, theme: &aylis_core::Theme) -> TextStyle {
    let mut st = one.clone();
    if let Some(weight) = span.weight {
        st.font.weight = weight;
    }
    if span.mono {
        st.font.family = "Cascadia Code".into();
    }
    st.font.italic |= span.italic;
    st.font.underline |= span.underline;
    st.font.size = span.size.unwrap_or(st.font.size) * span.scale;
    if let Some(color) = span.color.as_deref() {
        st.color = theme.colors.get(color).copied().or_else(|| aylis_core::Color::from_hex(color)).unwrap_or(st.color);
    }
    st
}

/// O espaço que as etiquetas (`<tag>`) somam à largura do texto: o espaçamento `xs` do tema
/// dos dois lados de cada uma.
fn markup_tag_room(text: &str, theme: &aylis_core::Theme) -> f32 {
    let tags = markup_spans(text).iter().filter_map(|s| s.tag.as_ref().map(|t| t.id)).max().map_or(0, |n| n + 1);
    tags as f32 * 2.0 * theme.spacing.get("xs").copied().unwrap_or(4.0)
}

/// Uma linha com `spacing` pixels a mais entre as letras, alinhada como o estilo pede.
pub fn draw_spaced(canvas: &mut dyn Canvas, text: &str, area: RectF, style: &TextStyle, spacing: f32) {
    let chars: Vec<String> = text.chars().map(String::from).collect();
    if chars.is_empty() {
        return;
    }
    let widths: Vec<f32> = chars.iter().map(|c| canvas.measure_text(c, style).width).collect();
    let total = widths.iter().sum::<f32>() + spacing * (chars.len() - 1) as f32;
    let mut x = match style.h_align {
        TextAlign::Leading | TextAlign::Justify => area.x,
        TextAlign::Center => area.x + (area.width - total) / 2.0,
        TextAlign::Trailing => area.right() - total,
    };
    let mut one = style.clone();
    one.h_align = TextAlign::Leading;
    one.wrap = false;
    one.trimming = Trimming::None;
    for (c, w) in chars.iter().zip(widths) {
        if x > area.right() {
            break;
        }
        canvas.draw_text(c, RectF::new(x, area.y, w + 2.0, area.height), &one);
        x += w + spacing;
    }
}

/// Uma letra sem acento e minúscula (para achar "sao" em "São").
fn fold_char(c: char) -> char {
    let c = c.to_lowercase().next().unwrap_or(c);
    match c {
        'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
        'é' | 'è' | 'ê' | 'ë' => 'e',
        'í' | 'ì' | 'î' | 'ï' => 'i',
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
        'ú' | 'ù' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        'ñ' => 'n',
        c => c,
    }
}

/// Os trechos de `line` que combinam com `query` (em caracteres: início, fim).
pub fn find_matches(line: &str, query: &str) -> Vec<(usize, usize)> {
    let hay: Vec<char> = line.chars().map(fold_char).collect();
    let needle: Vec<char> = query.trim().chars().map(fold_char).collect();
    let mut out = Vec::new();
    if needle.is_empty() || needle.len() > hay.len() {
        return out;
    }
    let mut i = 0;
    while i + needle.len() <= hay.len() {
        if hay[i..i + needle.len()] == needle[..] {
            out.push((i, i + needle.len()));
            i += needle.len();
        } else {
            i += 1;
        }
    }
    out
}

/// O fundo do trecho destacado: a cor `highlight` do tema, ou `warning` clara.
fn highlight_color(theme: &aylis_core::Theme) -> Color {
    theme.colors.get("highlight").copied().unwrap_or_else(|| theme.color("warning").with_opacity(0.35))
}

/// O texto linha a linha, com o fundo marcado onde aparece a busca (o `v-highlight`).
fn paint_highlighted(canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>, text: &str, query: &str, area: RectF, style: &aylis_core::TextStyle) {
    let line_h = (style.font.size * 1.35).round();
    let max_lines = if style.wrap { ((area.height / line_h).floor() as usize).max(1) } else { text.split('\n').count() };
    let lines = {
        let mut measure = |s: &str| canvas.measure_text(s, style).width;
        aylis_core::text::layout_lines(text, area.width, max_lines, style.wrap, style.trimming, &mut measure)
    };
    let total = lines.len() as f32 * line_h;
    let mut y = match style.v_align {
        TextAlign::Leading | TextAlign::Justify => area.y,
        TextAlign::Center => area.y + (area.height - total) / 2.0,
        TextAlign::Trailing => area.bottom() - total,
    };
    let mark = highlight_color(ctx.theme);
    let mut one = style.clone();
    one.wrap = false;
    one.trimming = Trimming::None;
    one.h_align = TextAlign::Leading;
    for line in &lines {
        let width = canvas.measure_text(line, style).width;
        let x = match style.h_align {
            TextAlign::Leading | TextAlign::Justify => area.x,
            TextAlign::Center => area.x + (area.width - width) / 2.0,
            TextAlign::Trailing => area.right() - width,
        };
        for (a, b) in find_matches(line, query) {
            let prefix: String = line.chars().take(a).collect();
            let part: String = line.chars().skip(a).take(b - a).collect();
            let x0 = x + canvas.measure_text(&prefix, style).width;
            let w = canvas.measure_text(&part, style).width;
            canvas.fill_rect(RectF::new(x0 - 1.0, y + 1.0, w + 2.0, line_h - 2.0), 3.0, &Brush::Solid(mark));
        }
        canvas.draw_text(line, RectF::new(x, y, width + 4.0, line_h), &one);
        y += line_h;
    }
}

// ---------------------------------------------------------------------------

/// Botão.
///
/// Toda a aparência vem do estilo `Button` do tema, incluindo os estados hover, pressed,
/// focused e disabled e a transição entre eles. A variante (`style`) escolhe outro
/// visual: `"danger"`, `"flat"`...
///
/// Clica com o mouse (soltando sobre o botão), com Enter ou com Espaço quando tem foco. O
/// Enter clica ao apertar e, seguro, repete o clique (`repeat_enter`);
/// o Espaço afunda o botão e clica ao soltar.
///
/// `icon` põe um ícone (pelo nome: os do aylis, como `"play"`, ou os registrados com
/// `Ui::register_icon`) ao lado do texto, ou sozinho se o texto for vazio. Com
/// `checkable`, cada clique alterna `checked`: o tema pinta
/// o estado `checked` e `checked_icon` troca o ícone (mudo/som, fixar/soltar).
pub struct Button {
    pub base: ControlBase,
    /// O texto do botão; `&` antes de uma letra faz dela o atalho com Alt. Vazio: só o ícone.
    pub text: String,
    /// Num form modal, clicar fecha o form com este resultado:
    /// `ok` num "Confirmar", `cancel` num "Cancelar".
    pub modal_result: ModalResult,
    /// Ícone pelo nome (`"play"` ou um registrado com `Ui::register_icon`); vazio: sem ícone.
    pub icon: String,
    /// Ícone quando marcado (vazio = o mesmo `icon`).
    pub checked_icon: String,
    /// Onde fica o ícone em relação ao texto: à esquerda, à direita ou em cima.
    pub icon_position: IconPosition,
    /// Tamanho do ícone; 0 = pelo tamanho da fonte (ou do botão, sem texto).
    pub icon_size: f32,
    /// Cada clique alterna o `checked` (um botão de ligar e desligar).
    pub checkable: bool,
    /// Marcado (afundado), com `checkable`; o tema pinta o estado `checked`.
    pub checked: bool,
    /// O Enter seguro repete o clique (a repetição do teclado). Desligado, só o
    /// primeiro Enter clica.
    pub repeat_enter: bool,
    /// O botão padrão do form: o Enter que nenhum controle usou o
    /// clica.
    pub default_button: bool,
    /// O botão de cancelar do form: o Esc que nenhum controle usou o
    /// clica.
    pub cancel_button: bool,
    /// Onde ficam o texto e o ícone: no centro (o padrão), à esquerda ou à direita (o item de um
    /// menu lateral, a linha de uma lista de botões).
    pub h_align: TextAlign,
    /// Grupo exclusivo: entre os botões `checkable` do mesmo
    /// pai com o mesmo número (> 0), marcar um desmarca os outros, e clicar no marcado não
    /// desmarca (a aba de uma barra, o modo de uma ferramenta). 0 = sem grupo.
    pub group_index: u32,
    /// Raio dos cantos: um número, ou `[cima-esquerda, cima-direita, baixo-direita, baixo-esquerda]`. Negativo curva o canto para fora (a aba que se funde no painel). Vazio: o desenho do tema.
    pub radius: Option<Bounds>,
    /// O Espaço foi apertado com o foco aqui: o clique vem ao soltar.
    space_down: bool,
}

impl Default for Button {
    fn default() -> Self {
        let mut base = ControlBase::new(96.0, 32.0);
        base.set_can_focus(true).set_cursor(CursorShape::Pointer);
        Self {
            base,
            text: String::new(),
            modal_result: ModalResult::None,
            icon: String::new(),
            checked_icon: String::new(),
            icon_position: IconPosition::Left,
            icon_size: 0.0,
            checkable: false,
            checked: false,
            repeat_enter: true,
            default_button: false,
            cancel_button: false,
            h_align: TextAlign::Center,
            group_index: 0,
            radius: None,
            space_down: false,
        }
    }
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
    }

    /// Botão só com ícone.
    pub fn with_icon(icon: impl Into<String>) -> Self {
        Self { icon: icon.into(), ..Default::default() }
    }

    fn sync(&mut self, _name: &str) {
        self.base.state_mut().checked = self.checked;
    }

    fn current_icon(&self) -> &str {
        if self.checked && !self.checked_icon.is_empty() { &self.checked_icon } else { &self.icon }
    }

    /// O Espaço apertado ou solto: o botão aparece afundado enquanto ele está apertado.
    fn set_space_down(&mut self, ctx: &mut EventCtx<'_>, down: bool) {
        self.space_down = down;
        self.base.state_mut().pressed = down;
        ctx.repaint();
    }
}

impl Component for Button {
    fn layout_hooks(&self) -> Option<&dyn aylis_core::LayoutHooks> {
        Some(self)
    }

    /// O `radius` próprio: o Reveal acende pelos mesmos cantos que o fundo.
    fn painted_shape(&self) -> Option<aylis_core::ClipShape> {
        self.radius.map(aylis_core::ClipShape::from_corners)
    }

    leaf_control!("Button");

    fn checked_state(&self) -> Option<bool> {
        self.checkable.then_some(self.checked)
    }

    fn mnemonic_text(&self) -> Option<&str> {
        Some(&self.text)
    }

    properties! {
        text: String => "Appearance",
        icon: String => "Appearance",
        checked_icon: String => "Appearance",
        icon_position: IconPosition => "Appearance",
        icon_size: f32 => "Appearance",
        checkable: bool => "Behavior",
        checked: bool => "Data",
        modal_result: ModalResult => "Behavior",
        repeat_enter: bool => "Behavior",
        default_button: bool => "Behavior",
        cancel_button: bool => "Behavior",
        h_align: TextAlign => "Appearance",
        group_index: u32 => "Behavior",
        radius: Option<Bounds> => "Appearance";
        events: on_change: bool;
        on_set: sync
    }

    // O `on_change` é do botão marcável; o principal é o clique.
    fn default_event(&self) -> Option<&'static str> {
        Some("on_click")
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        // Num grupo, o marcado continua marcado; o novo desmarca os irmãos do grupo (C03 do
        // Tabernáculo).
        let grouped = self.checkable && self.group_index > 0;
        if self.checkable && !(grouped && self.checked) {
            self.checked = !self.checked;
            self.sync("checked");
            ctx.repaint();
            ctx.fire("on_change", EventArgs::Value(self.checked.into()));
            if grouped {
                let (me, group) = (ctx.id, self.group_index);
                let siblings: Vec<ComponentId> = ctx.ui.parent(me).map(|p| ctx.ui.children(p).to_vec()).unwrap_or_default();
                for other in siblings.into_iter().filter(|&o| o != me) {
                    let same = matches!(ctx.ui.get_prop(other, "group_index"), Ok(aylis_core::PropValue::Int(g)) if g == group as i64);
                    if same && matches!(ctx.ui.get_prop(other, "checked"), Ok(aylis_core::PropValue::Bool(true))) {
                        let _ = ctx.ui.set_prop(other, "checked", false);
                    }
                }
            }
        }
        ctx.fire("on_click", EventArgs::None);
        if self.modal_result != ModalResult::None {
            // O form fecha depois do on_click (que ainda pode cancelar).
            let form = ctx.ui.form_of(ctx.id);
            ctx.ui.set_modal_result(form, self.modal_result);
        }
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let style = ctx.style;
        // Com fundo próprio (`fill`, de todo controle), o desenho da plataforma (o degradê do
        // Windows 7...) dá lugar à cor.
        match self.radius {
            // Raio próprio (um por canto, ou negativo): o fundo pelo contorno, na cor do estado.
            Some(corners) => {
                let path = aylis_core::Path::corners(ctx.rect, corners);
                canvas.fill_path(&path, &Brush::Solid(style.fill));
                if style.border_width > 0.0 && !style.border_color.is_transparent() {
                    canvas.stroke_path(&path, &Stroke::new(style.border_color, style.border_width));
                }
            }
            None => crate::looks::paint_button_fill(canvas, ctx, if ctx.colors.fill.is_some() { "custom" } else { ctx.variant() }),
        }
        let mut text = ctx.text_style();
        text.h_align = self.h_align;
        let mut inner = ctx.rect.deflate(style.padding);
        // "&Salvar": Alt+S clica (o & some e a letra fica sublinhada).
        let (plain, at) = crate::mnemonic::caption(&self.text);
        let label = crate::looks::button_text(ctx.look(), &plain);
        // A caixa-alta e o espaçamento do tema (o relatório do Tabernáculo, C09).
        let label = style.text_case.apply(label.as_ref());
        let label = label.as_str();
        let spacing = style.letter_spacing;
        let icon = self.current_icon();
        if icon.is_empty() {
            // Botão estreito: o espaço interno cede antes de cortar o texto (um "+" num botão
            // de 28 px não vira "…").
            let need = canvas.measure_text(label, &text).width;
            if need > inner.width && inner.width < ctx.rect.width {
                let room = need.min(ctx.rect.width - 2.0).max(inner.width);
                inner.x = ctx.rect.x + (ctx.rect.width - room) / 2.0;
                inner.width = room;
            }
            if spacing != 0.0 {
                draw_spaced(canvas, label, inner, &text, spacing);
            } else {
                crate::mnemonic::draw_text(canvas, label, at, inner, &text);
            }
            return;
        }
        if label.is_empty() {
            let side = if self.icon_size > 0.0 { self.icon_size } else { inner.width.min(inner.height).min(text.font.size * 1.6).max(1.0) };
            let r = RectF::new(inner.x + (inner.width - side) / 2.0, inner.y + (inner.height - side) / 2.0, side, side);
            ctx.draw_icon(canvas, icon, r, style.text);
            return;
        }
        // Ícone e texto juntos, centralizados.
        let side = if self.icon_size > 0.0 { self.icon_size } else { (text.font.size * 1.25).round() };
        let gap = 6.0;
        let text_width = canvas.measure_text(label, &text).width.min(inner.width - side - gap).max(0.0);
        match self.icon_position {
            IconPosition::Top => {
                let line = text.font.size * 1.4;
                let total = side + 2.0 + line;
                let top = inner.y + (inner.height - total) / 2.0;
                ctx.draw_icon(canvas, icon, RectF::new(inner.x + (inner.width - side) / 2.0, top, side, side), style.text);
                crate::mnemonic::draw_text(canvas, label, at, RectF::new(inner.x, top + side + 2.0, inner.width, line), &text);
            }
            pos => {
                let total = side + gap + text_width;
                // Ícone e texto juntos, no lugar do `h_align` (C13 do Tabernáculo).
                let left = match self.h_align {
                    TextAlign::Leading | TextAlign::Justify => inner.x,
                    TextAlign::Trailing => (inner.right() - total).max(inner.x),
                    TextAlign::Center => inner.x + ((inner.width - total) / 2.0).max(0.0),
                };
                let icon_y = inner.y + (inner.height - side) / 2.0;
                text.h_align = TextAlign::Leading;
                let (icon_x, text_x) = if pos == IconPosition::Left { (left, left + side + gap) } else { (left + text_width + gap, left) };
                ctx.draw_icon(canvas, icon, RectF::new(icon_x, icon_y, side, side), style.text);
                crate::mnemonic::draw_text(canvas, label, at, RectF::new(text_x, inner.y, text_width + 1.0, inner.height), &text);
            }
        }
    }

    fn key_down(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        match event.key {
            Key::Enter => {
                if !event.repeat || self.repeat_enter {
                    self.click(ctx);
                }
                ctx.set_handled();
            }
            // Clica ao soltar: o Espaço que abriu este form (apertado em outro) não clica aqui.
            Key::Space => {
                if !event.repeat {
                    self.set_space_down(ctx, true);
                }
                ctx.set_handled();
            }
            _ => {}
        }
    }

    fn key_up(&mut self, ctx: &mut EventCtx<'_>, event: &KeyEvent) {
        if event.key == Key::Space && self.space_down {
            self.set_space_down(ctx, false);
            self.click(ctx);
            ctx.set_handled();
        }
    }

    fn focus_exit(&mut self, ctx: &mut EventCtx<'_>) {
        // Perdeu o foco com o Espaço apertado: solta sem clicar.
        if self.space_down {
            self.set_space_down(ctx, false);
        }
    }
}

impl aylis_core::LayoutHooks for Button {
    fn baseline(&self, ui: &aylis_core::Ui, id: ComponentId, height: f32) -> Option<f32> {
        Some(text_baseline(ui, id, height, ui.style_padding(id), TextAlign::Center))
    }

    /// `auto_size`: a largura do texto e do ícone mais o padding do estilo (antes o botão
    /// ficava nos 96 de fábrica; o relatório do Tabernáculo, C14).
    fn auto_width(&self, ui: &aylis_core::Ui, id: ComponentId) -> Option<f32> {
        let style = TextStyle { font: ui.font_of(id), ..Default::default() };
        let (plain, _) = crate::mnemonic::caption(&self.text);
        let pad = ui.style_padding(id);
        let text = if plain.is_empty() { 0.0 } else { ui.measure_text(&plain, &style).width };
        let icon = if self.current_icon().is_empty() { 0.0 } else if self.icon_size > 0.0 { self.icon_size } else { (style.font.size * 1.25).round() };
        let gap = if text > 0.0 && icon > 0.0 && self.icon_position != IconPosition::Top { 6.0 } else { 0.0 };
        let content = if self.icon_position == IconPosition::Top { text.max(icon) } else { text + gap + icon };
        Some((content + pad.horizontal() + 2.0).ceil())
    }

    fn auto_height(&self, ui: &aylis_core::Ui, id: ComponentId, _width: f32) -> Option<f32> {
        let style = TextStyle { font: ui.font_of(id), ..Default::default() };
        let pad = ui.style_padding(id);
        let line = ui.measure_text("Ág", &style).height;
        let icon = if self.current_icon().is_empty() { 0.0 } else if self.icon_size > 0.0 { self.icon_size } else { (style.font.size * 1.25).round() };
        let content = if self.icon_position == IconPosition::Top { icon + 2.0 + line } else { line.max(icon) };
        Some((content + pad.vertical()).ceil())
    }
}

// ---------------------------------------------------------------------------

/// Barra de progresso. O trilho usa o `fill` do tema e a barra,
/// o token `primary` (ou `color`, se definido).
pub struct ProgressBar {
    pub base: ControlBase,
    /// O valor atual, entre `min` e `max`: quanto da barra fica preenchido.
    pub value: f32,
    /// O valor da barra vazia.
    pub min: f32,
    /// O valor da barra cheia.
    pub max: f32,
    /// Cor da barra; vazio: o token `primary` do tema.
    pub color: Option<Color>,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self { base: ControlBase::new(200.0, 8.0), value: 0.0, min: 0.0, max: 100.0, color: None }
    }
}

impl ProgressBar {
    /// Fração preenchida, de 0 a 1.
    pub fn fraction(&self) -> f32 {
        if self.max > self.min { ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0) } else { 0.0 }
    }
}

impl Component for ProgressBar {
    // O valor só muda a pintura: não refaz o layout da tela.
    fn paint_only_property(&self, name: &str) -> bool {
        matches!(name, "value")
    }

    leaf_control!("ProgressBar");

    properties! {
        value: f32 => "Data",
        min: f32 => "Data",
        max: f32 => "Data",
        color: Option<Color> => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let r = ctx.rect;
        let radius = ctx.style.radius.min(r.height / 2.0);
        canvas.fill_rect(r, radius, &Brush::Solid(ctx.style.fill));
        let width = r.width * self.fraction();
        if width > 0.0 {
            // As variantes `success`, `danger` e `warning` mudam a cor da barra.
            let token = match ctx.variant() {
                v @ ("success" | "danger" | "warning") => v,
                // O tema pode ter a cor própria da barra (a verde do Win32).
                _ if ctx.theme.colors.contains_key("progress") => "progress",
                _ => "primary",
            };
            let color = self.color.unwrap_or_else(|| ctx.theme.color(token));
            let bar = RectF::new(r.x, r.y, width.max(radius * 2.0), r.height);
            canvas.fill_rect(bar, radius, &Brush::Solid(color));
            crate::looks::paint_progress_gloss(canvas, ctx, bar);
        }
    }
}
