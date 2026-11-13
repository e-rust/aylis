//! Controles simples: Form, Layout, Frame, Rectangle, Panel, Label, Button e ProgressBar.

use aylis_core::{
    Bounds, Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent, LayerCaching,
    GradientStop, ModalResult, PaintCtx, PointF, PropEnum, RectF, Stroke, TextAlign, TextCase, TextStyle, Transition, Trimming, properties,
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

/// Onde a janela do form abre (como o `Position` do `TForm`).
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

/// Como a janela do form fica (como o `WindowState` do `TForm`, mais a tela cheia do
/// `FullScreen` do FMX). Vale ao abrir e, com o app rodando, quando muda.
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

/// Borda da janela (como o `BorderStyle` do `TForm`).
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

impl PropEnum for BorderStyle {
    const NAMES: &'static [&'static str] = &["sizeable", "single", "none", "custom"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Sizeable, Self::Single, Self::None, Self::Custom].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Janela (como o `TForm`). Pinta o fundo com o estilo `Form` do tema.
///
/// Abrir e fechar: [`Ui::show`](aylis_core::Ui::show), [`Ui::show_modal`](aylis_core::Ui::show_modal)
/// e [`Ui::close`](aylis_core::Ui::close). Um form também pode ser embutido em outro
/// (`ui.set_parent(form, Some(painel))`, como `Form2.Parent := Panel1`).
pub struct Form {
    pub base: ControlBase,
    pub caption: String,
    pub position: FormPosition,
    pub border_style: BorderStyle,
    /// Os cantos da janela (Windows 11 e macOS; nas outras, sem efeito): útil com a barra de
    /// título própria (`border_style = custom`), que fica com os cantos de uma janela comum.
    pub window_corners: WindowCorners,
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
    /// Sempre por cima das outras janelas (o `FormStyle := fsStayOnTop`).
    pub topmost: bool,
    /// Aparece na barra de tarefas (desligue para menus e janelas de ferramenta).
    pub show_in_taskbar: bool,
    /// O controle que recebe o foco quando o form abre (como o `ActiveControl`).
    pub active_control: Option<ComponentId>,
    /// A ordem do Tab no form inteiro: nomes separados por `;` (`edtNome;edtEmail;btnSalvar`).
    /// Pula entre níveis (de um campo num painel para outro num grupo); os controles fora
    /// da lista vêm depois, na ordem natural. Vazio: a ordem natural (a da árvore).
    pub tab_order: String,
    /// As teclas que fazem o papel do Tab (`"Enter"`, `"Enter;Down"`), como o Enter que pula
    /// de campo nos sistemas em Delphi. O controle que usa a tecla (o Enter num Memo) fica
    /// com ela.
    pub tab_keys: String,
    /// As que fazem o papel do Shift+Tab (`"Shift+Enter;Up"`).
    pub back_tab_keys: String,
    /// Os caracteres liberados, bloqueados e só os liberados, para todos os campos do form
    /// (o campo decide primeiro; `chars_override` faz o form mandar). Faixas: `0-9`, `a-z`.
    pub allowed_chars: String,
    pub blocked_chars: String,
    pub only_allowed: bool,
    pub chars_override: bool,
    /// Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a
    /// mesma herança dos filtros.
    pub char_map: String,
    /// O form recebe as teclas antes do controle com foco (`on_key_down` e `on_key_press`), como
    /// o `KeyPreview` do Delphi.
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
    /// Mostrar a janela dá o foco a ela (desligado: aparece sem tirar o foco de quem tem, como
    /// uma projeção que não deve roubar o teclado do console).
    pub activate_on_show: bool,
    /// O que fechar faz com o form (o `Action` do `OnClose` do Delphi): esconde (pode abrir
    /// de novo) ou libera (some da memória, depois de fechar).
    pub close_action: CloseAction,
    /// Quadros por segundo da janela (0 = a taxa do monitor dela). Um telão de 60 Hz ao lado
    /// de um monitor de 144 Hz não precisa acompanhar o mais rápido.
    pub frame_rate: f32,
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
    /// A largura da área cliente (o `ClientWidth` do Delphi): no Aylis, o `width` do form já é
    /// a área de dentro, sem a moldura e a barra de título do sistema.
    pub fn client_width(&self) -> f32 {
        self.base.width()
    }

    pub fn set_client_width(&mut self, width: f32) {
        let height = self.base.height();
        self.base.set_size(width.max(0.0), height);
    }

    /// A altura da área cliente (o `ClientHeight`), sem a barra de título.
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
            window_state: WindowState::Normal,
            transition: Transition::None,
            layer_cache: LayerCaching::Project,
            topmost: false,
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
            activate_on_show: true,
            close_action: CloseAction::Hide,
            frame_rate: 0.0,
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
        style.fill.a == 255
    }

    leaf_control!("Form");

    properties! {
        caption: String => "Appearance",
        position: FormPosition => "Layout",
        border_style: BorderStyle => "Appearance",
        window_corners: WindowCorners => "Appearance",
        window_state: WindowState => "Layout",
        transition: Transition => "Behavior",
        layer_cache: LayerCaching => "Behavior",
        topmost: bool => "Behavior",
        show_in_taskbar: bool => "Behavior",
        active_control: Option<ComponentId> => "Behavior",
        tab_order: String => "Behavior",
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
        activate_on_show: bool => "Behavior",
        close_action: CloseAction => "Behavior",
        frame_rate: f32 => "Behavior";
        events: on_show, on_window_created, on_close, on_close_query, on_activate, on_deactivate, on_minimize, on_restore, on_navigate, on_gpu_change;
        computed: client_width: f32 => client_width / set_client_width, client_height: f32 => client_height / set_client_height
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::{MethodInfo, PropKind};
        const METHODS: &[MethodInfo] = &[
            MethodInfo::new("move_to_monitor", &[PropKind::Int], None).params(&["monitor"]),
            MethodInfo::new("gpu_adapter", &[], Some(PropKind::String)),
            MethodInfo::new("gpu_adapters", &[], Some(PropKind::String)),
        ];
        METHODS
    }

    /// `move_to_monitor(i)`: leva a janela para o monitor `i` (o índice do `Screen`).
    fn invoke(&mut self, ctx: &mut EventCtx<'_>, method: &str, args: &[aylis_core::PropValue]) -> Result<aylis_core::PropValue, String> {
        match method {
            "move_to_monitor" => {
                let index = args.first().and_then(|v| v.to_i64().ok()).filter(|i| *i >= 0).ok_or("move_to_monitor pede o índice do monitor")?;
                ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::MoveToMonitor(index as usize));
                Ok(aylis_core::PropValue::Null)
            }
            // A placa em que a janela desenha agora (vazio: não se sabe, como na CPU).
            "gpu_adapter" => Ok(aylis_core::PropValue::String(ctx.ui.form_gpu(ctx.id).unwrap_or_default().to_string())),
            // As placas da máquina, separadas por `;` (o texto que a propriedade `gpu` aceita).
            "gpu_adapters" => Ok(aylis_core::PropValue::String(aylis_core::gpu::adapters().join(";"))),
            _ => Err(format!("Form não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "zoom" {
            ctx.ui.mark_zoom_changed();
        }
        // Com o app rodando, a janela segue (no designer não há janela).
        if name == "gpu" && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateGpu);
        }
        if matches!(name, "window_corners" | "border_style") && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::UpdateCorners);
        }
        // O tamanho mudado pelo código com o app rodando: a janela acompanha, com a área
        // cliente do tamanho pedido (a moldura fica por fora; J11).
        if matches!(name, "width" | "height" | "client_width" | "client_height") && !ctx.ui.design_mode() {
            ctx.ui.window_command(ctx.id, aylis_core::WindowCommand::ApplySize);
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
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        canvas.fill_rect(ctx.rect, 0.0, &Brush::Solid(ctx.style.fill));
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

/// Contêiner invisível, só para organizar o layout (como o `TLayout`).
pub struct Layout {
    pub base: ControlBase,
    /// Os caracteres dos campos dentro do layout (veja o `Form`).
    pub allowed_chars: String,
    pub blocked_chars: String,
    pub only_allowed: bool,
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
    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    leaf_control!("Layout");

    fn dock_gap(&self) -> f32 {
        self.gap
    }

    properties! {
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        chars_override: bool => "Keyboard",
        char_map: String => "Keyboard",
        gap: f32 => "Layout",
    }
}

// ---------------------------------------------------------------------------

/// Um form do projeto dentro de outro (como o `TFrame` do Delphi): `form` é o nome dele.
/// Cada parte da tela fica no seu arquivo, desenhada e editada à parte, e o form que usa
/// só reserva o lugar. Ao carregar o projeto, a fábrica de forms cria o form do frame e o
/// põe aqui, ocupando a área toda (o fundo dele fica transparente, se ele não tiver um);
/// no designer, a IDE mostra o conteúdo sem deixar mexer nele. Os componentes do frame se
/// acham pelo nome do form dele (`frmConfig.edtPasta`) e os eventos vão para os handlers
/// desse form.
pub struct Frame {
    pub base: ControlBase,
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
        form: String => "Behavior",
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

/// Retângulo (como o `TRectangle`). Cada propriedade vazia (`Null`) usa o valor do tema.
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
    pub stroke: Option<Color>,
    pub stroke_width: Option<f32>,
    /// Raio dos cantos: um número, ou `[cima-esquerda, cima-direita, baixo-direita, baixo-esquerda]`. Negativo curva o canto para fora (a aba que se funde no painel). Vazio: o do tema.
    pub radius: Option<Bounds>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self { base: ControlBase::new(width, height), ..Default::default() }
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
        radius: Option<Bounds> => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let corners = self.radius.unwrap_or(aylis_core::Bounds::all(ctx.style.radius));
        let fill = self.fill.unwrap_or(ctx.style.fill);
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
                canvas.stroke_path(&path, &ctx.style.border_stroke(stroke, width));
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
        stroke_style_border(canvas, ctx.rect, radius, stroke, width, ctx.style);
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
    pub blocked_chars: String,
    pub only_allowed: bool,
    pub chars_override: bool,
    /// Trocas do que o teclado produz nos campos de dentro (`",."`: vírgula por ponto), com a
    /// mesma herança dos filtros.
    pub char_map: String,
    /// O vão entre os filhos encaixados (o `gap` do CSS; L05 do Tabernáculo).
    pub gap: f32,
}

impl Panel {
    pub fn new(width: f32, height: f32) -> Self {
        Self { base: Rectangle::new(width, height), ..Default::default() }
    }
}

impl Component for Panel {
    /// Só fundo e borda pelo estilo: o ensaio do desenho (`Ui::drawing_unchanged`) vale.
    fn paint_is_pure(&self) -> bool {
        true
    }

    fn class_name(&self) -> &'static str {
        "Panel"
    }

    fn dock_gap(&self) -> f32 {
        self.gap
    }

    properties! {
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        chars_override: bool => "Keyboard",
        char_map: String => "Keyboard",
        gap: f32 => "Layout",
    }

    fn inner(&self) -> Option<&dyn Component> {
        Some(&self.base)
    }

    fn inner_mut(&mut self) -> Option<&mut dyn Component> {
        Some(&mut self.base)
    }
}

// ---------------------------------------------------------------------------

/// Texto (como o `TLabel`).
pub struct Label {
    pub base: ControlBase,
    pub text: String,
    pub h_align: TextAlign,
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
    /// O campo que recebe o foco no Alt+letra do `&` do texto (o `FocusControl` do Delphi).
    pub focus_control: Option<ComponentId>,
    /// O texto traz marcas simples no meio, que viram estilo em vez de aparecer (os textos com
    /// HTML simples de um app web): `<b>` negrito, `<sb>` seminegrito, `<i>` itálico, `<u>`
    /// sublinhado, `<code>` fonte mono, `<small>` e `<big>` (85% e 120%), `<size=18>`,
    /// `<color=text_muted>` (token do tema ou `#rrggbb`) e `<br>` (quebra). Ver [`markup_spans`].
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

    /// O texto quebrado com a altura de linha `line_height` (em vezes a fonte), linha a linha.
    fn paint_lines(canvas: &mut dyn Canvas, text: &str, area: RectF, style: &TextStyle, line_height: f32) {
        let mut one = style.clone();
        one.wrap = false;
        one.v_align = TextAlign::Center;
        let mut measure = |s: &str| canvas.measure_text(s, &one).width;
        let lines = aylis_core::text::layout_lines(text, area.width, usize::MAX, true, style.trimming, &mut measure);
        let step = style.font.size * line_height;
        let total = step * lines.len() as f32;
        let mut y = match style.v_align {
            TextAlign::Leading | TextAlign::Justify => area.y,
            TextAlign::Center => area.y + ((area.height - total) / 2.0).max(0.0),
            TextAlign::Trailing => area.y + (area.height - total).max(0.0),
        };
        for line in &lines {
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

    fn baseline(&self, ui: &aylis_core::Ui, id: ComponentId, height: f32) -> Option<f32> {
        let pad = add_bounds(ui.style_padding(id), self.base.padding());
        Some(text_baseline(ui, id, height, pad, self.v_align))
    }

    fn notification(&mut self, freed: ComponentId) {
        if self.focus_control == Some(freed) {
            self.focus_control = None;
        }
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
        // Com a altura de linha própria: as linhas quebradas vezes ela.
        if line_height > 0.0 && self.wrap && !self.markup {
            let mut measure = |s: &str| ui.measure_text(s, &style).width;
            let lines = aylis_core::text::layout_lines(&text, inner, usize::MAX, true, Trimming::None, &mut measure).len().max(1);
            return Some((lines as f32 * style.font.size * line_height).ceil() + pad.vertical() + 1.0);
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
        Some((ui.measure_text(&text, &style).width * if self.markup { 1.04 } else { 1.0 } + pad.horizontal() + 2.0).ceil())
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
        if line_height > 0.0 && self.wrap && !self.markup && letter_spacing == 0.0 {
            let (text, _) = crate::mnemonic::caption(&text_case.apply(&self.text));
            Self::paint_lines(canvas, &text, area, &style, line_height);
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
}

impl MarkupSpan {
    fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), weight: None, italic: false, underline: false, mono: false, color: None, size: None, scale: 1.0 }
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
        let known = matches!(name, "b" | "sb" | "i" | "u" | "code" | "small" | "big" | "size" | "color" | "br");
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
    for span in runs {
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
    let mark = ctx.theme.colors.get("highlight").copied().unwrap_or_else(|| ctx.theme.color("warning").with_opacity(0.35));
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

/// Botão (como o `TButton`).
///
/// Toda a aparência vem do estilo `Button` do tema, incluindo os estados hover, pressed,
/// focused e disabled e a transição entre eles. A variante (`style`) escolhe outro
/// visual: `"danger"`, `"flat"`...
///
/// Clica com o mouse (soltando sobre o botão), com Enter ou com Espaço quando tem foco. Como
/// no `TButton` da VCL, o Enter clica ao apertar e, seguro, repete o clique (`repeat_enter`);
/// o Espaço afunda o botão e clica ao soltar.
///
/// `icon` põe um ícone (pelo nome: os do aylis, como `"play"`, ou os registrados com
/// `Ui::register_icon`) ao lado do texto, ou sozinho se o texto for vazio. Com
/// `checkable`, cada clique alterna `checked` (como o `StaysPressed` do FMX): o tema pinta
/// o estado `checked` e `checked_icon` troca o ícone (mudo/som, fixar/soltar).
pub struct Button {
    pub base: ControlBase,
    pub text: String,
    /// Num form modal, clicar fecha o form com este resultado (como o `ModalResult` do
    /// `TButton`): `ok` num "Confirmar", `cancel` num "Cancelar".
    pub modal_result: ModalResult,
    pub icon: String,
    /// Ícone quando marcado (vazio = o mesmo `icon`).
    pub checked_icon: String,
    pub icon_position: IconPosition,
    /// Tamanho do ícone; 0 = pelo tamanho da fonte (ou do botão, sem texto).
    pub icon_size: f32,
    pub checkable: bool,
    pub checked: bool,
    /// O Enter seguro repete o clique (a repetição do teclado), como na VCL. Desligado, só o
    /// primeiro Enter clica.
    pub repeat_enter: bool,
    /// O botão padrão do form (o `Default` do `TButton`): o Enter que nenhum controle usou o
    /// clica.
    pub default_button: bool,
    /// O botão de cancelar do form (o `Cancel` do `TButton`): o Esc que nenhum controle usou o
    /// clica.
    pub cancel_button: bool,
    /// Onde ficam o texto e o ícone: no centro (o padrão), à esquerda ou à direita (o item de um
    /// menu lateral, a linha de uma lista de botões).
    pub h_align: TextAlign,
    /// Grupo exclusivo (o `GroupIndex` do `TSpeedButton`): entre os botões `checkable` do mesmo
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
    leaf_control!("Button");

    fn baseline(&self, ui: &aylis_core::Ui, id: ComponentId, height: f32) -> Option<f32> {
        Some(text_baseline(ui, id, height, ui.style_padding(id), TextAlign::Center))
    }

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
        events: on_change;
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
            // O form fecha depois do on_click (que ainda pode cancelar), como no Delphi.
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

// ---------------------------------------------------------------------------

/// Barra de progresso (como o `TProgressBar`). O trilho usa o `fill` do tema e a barra,
/// o token `primary` (ou `color`, se definido).
pub struct ProgressBar {
    pub base: ControlBase,
    pub value: f32,
    pub min: f32,
    pub max: f32,
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
