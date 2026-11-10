//! Controles simples: Form, Layout, Frame, Rectangle, Panel, Label, Button e ProgressBar.

use aylis_core::{
    Brush, Canvas, Color, Component, ComponentId, ControlBase, CursorShape, EventArgs, EventCtx, Key, KeyEvent,
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
}

impl PropEnum for FormPosition {
    const NAMES: &'static [&'static str] = &["center", "screen_center", "designed", "default"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Center, Self::ScreenCenter, Self::Designed, Self::Default].get(index).copied()
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
    /// Normal, maximizada, minimizada ou em tela cheia (`full_screen`). No celular e no
    /// navegador o form já ocupa a tela toda.
    pub window_state: WindowState,
    /// Celular e navegador (os forms se empilham): como o form entra ao abrir e sai ao
    /// fechar (`slide_left` como uma tela nova do Android/iOS, `slide_up` como uma folha,
    /// `fade`). No desktop cada form é uma janela e isto não vale.
    pub transition: Transition,
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
}

impl Default for Form {
    fn default() -> Self {
        Self::new("", 640.0, 480.0)
    }
}

impl Form {
    pub fn new(caption: impl Into<String>, width: f32, height: f32) -> Self {
        Self {
            base: ControlBase::new(width, height),
            caption: caption.into(),
            position: FormPosition::default(),
            border_style: BorderStyle::default(),
            window_state: WindowState::Normal,
            transition: Transition::None,
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
        window_state: WindowState => "Layout",
        transition: Transition => "Behavior",
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
        remember_placement: bool => "Behavior";
        events: on_show, on_close, on_close_query, on_activate, on_deactivate, on_navigate
    }

    fn methods(&self) -> &'static [aylis_core::MethodInfo] {
        use aylis_core::{MethodInfo, PropKind};
        const METHODS: &[MethodInfo] = &[MethodInfo::new("move_to_monitor", &[PropKind::Int], None).params(&["monitor"])];
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
            _ => Err(format!("Form não tem o método '{method}'")),
        }
    }

    fn property_changed(&mut self, ctx: &mut EventCtx<'_>, name: &str) {
        if name == "zoom" {
            ctx.ui.mark_zoom_changed();
        }
        // Com o app rodando, a janela segue (no designer não há janela).
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
}

impl Default for Layout {
    fn default() -> Self {
        let mut base = ControlBase::new(100.0, 100.0);
        base.set_hit_test(false);
        Self { base, allowed_chars: String::new(), blocked_chars: String::new(), only_allowed: false, chars_override: false, char_map: String::new() }
    }
}

impl Component for Layout {
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
    pub radius: Option<f32>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self { base: ControlBase::new(width, height), ..Default::default() }
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
        self.fill2.is_none() && self.fill.unwrap_or(style.fill).a == 255 && self.radius.unwrap_or(style.radius) <= 0.0
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
        radius: Option<f32> => "Appearance",
    }

    fn paint(&self, canvas: &mut dyn Canvas, ctx: &PaintCtx<'_>) {
        let radius = self.radius.unwrap_or(ctx.style.radius);
        let fill = self.fill.unwrap_or(ctx.style.fill);
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
        if width > 0.0 && !stroke.is_transparent() {
            canvas.stroke_rect(ctx.rect, radius, &Stroke::new(stroke, width));
        }
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

    properties! {
        allowed_chars: String => "Keyboard",
        blocked_chars: String => "Keyboard",
        only_allowed: bool => "Keyboard",
        chars_override: bool => "Keyboard",
        char_map: String => "Keyboard",
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
    /// O texto traz `<b>…</b>` (negrito) e `<code>…</code>` (fonte mono) no meio: as
    /// marcas viram estilo em vez de aparecer (os textos com HTML simples de um app web).
    pub markup: bool,
    /// Contorno em volta das letras (a legenda sobre vídeo: legível em qualquer fundo).
    pub outline: Option<Color>,
    /// Espessura do contorno, em pixels (0 = sem contorno).
    pub outline_width: f32,
    /// Espaço a mais entre as letras, em pixels (os títulos em caixa-alta espaçados).
    pub letter_spacing: f32,
    /// Maiúsculas ou minúsculas ao desenhar (o texto não muda).
    pub text_case: TextCase,
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
        }
    }
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Default::default() }
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
        focus_control: Option<ComponentId> => "Behavior",
    }

    fn mnemonic_text(&self) -> Option<&str> {
        Some(&self.text)
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
        let pad = ui.style_padding(id);
        let style = TextStyle { font, wrap: self.wrap, ..Default::default() };
        // O negrito do markup é um pouco mais largo que o texto puro: mede com folga.
        let inner = (width - pad.horizontal()).max(1.0) * if self.markup { 0.97 } else { 1.0 };
        let size = if self.wrap { ui.measure_text_wrapped(&text, &style, inner) } else { ui.measure_text(&text, &style) };
        Some(size.height + pad.vertical() + 1.0)
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
        let area = ctx.rect.deflate(ctx.style.padding);
        if let Some(color) = self.outline.filter(|_| self.outline_width > 0.0 && !self.markup) {
            // O texto repetido em volta, na cor do contorno, antes do texto em si.
            let mut halo = style.clone();
            halo.color = color;
            let (text, _) = crate::mnemonic::caption(&self.text_case.apply(&self.text));
            let w = self.outline_width;
            for ring in [w, w * 0.5] {
                for k in 0..16 {
                    let a = k as f32 * std::f32::consts::TAU / 16.0;
                    canvas.draw_text(&text, area.offset(a.cos() * ring, a.sin() * ring), &halo);
                }
            }
        }
        if self.markup {
            paint_markup(canvas, &self.text, area, &style);
        } else if self.letter_spacing != 0.0 && !self.wrap {
            let (text, _) = crate::mnemonic::caption(&self.text);
            draw_spaced(canvas, &self.text_case.apply(&text), area, &style, self.letter_spacing);
        } else if self.highlight.trim().is_empty() {
            let (text, at) = crate::mnemonic::caption(&self.text_case.apply(&self.text));
            crate::mnemonic::draw_text(canvas, &text, at, area, &style);
        } else {
            paint_highlighted(canvas, ctx, &self.text, &self.highlight, area, &style);
        }
    }
}

/// Os trechos do texto com marcas: (texto, negrito, mono).
pub fn markup_runs(text: &str) -> Vec<(String, bool, bool)> {
    let mut runs = Vec::new();
    let (mut bold, mut mono) = (false, false);
    let mut rest = text;
    while !rest.is_empty() {
        let next = ["<b>", "</b>", "<code>", "</code>", "<br>", "<br/>"]
            .iter()
            .filter_map(|t| rest.find(t).map(|i| (i, *t)))
            .min_by_key(|(i, _)| *i);
        let Some((i, tag)) = next else {
            runs.push((rest.to_string(), bold, mono));
            break;
        };
        if i > 0 {
            runs.push((rest[..i].to_string(), bold, mono));
        }
        match tag {
            "<b>" => bold = true,
            "</b>" => bold = false,
            "<code>" => mono = true,
            "</code>" => mono = false,
            _ => runs.push(("\n".to_string(), bold, mono)),
        }
        rest = &rest[i + tag.len()..];
    }
    runs
}

/// O texto sem as marcas (para medir).
pub fn markup_plain(text: &str) -> String {
    markup_runs(text).into_iter().map(|(t, _, _)| t).collect()
}

/// Texto com negrito/mono no meio, quebrando nas palavras na largura da área.
fn paint_markup(canvas: &mut dyn Canvas, text: &str, area: RectF, style: &TextStyle) {
    let mut one = style.clone();
    one.h_align = TextAlign::Leading;
    one.v_align = TextAlign::Leading;
    one.wrap = false;
    let line_h = canvas.measure_text("Ág", &one).height.max(1.0);
    let space = (canvas.measure_text("a a", &one).width - canvas.measure_text("aa", &one).width).max(1.0);
    let (mut x, mut y) = (area.x, area.y);
    // Primeiro as posições; depois o desenho, já com o alinhamento vertical.
    let mut placed: Vec<(String, TextStyle, f32, f32, f32)> = Vec::new();
    for (run, bold, mono) in markup_runs(text) {
        let mut st = one.clone();
        if bold {
            st.font.weight = aylis_core::FontWeight::Bold;
        }
        if mono {
            st.font.family = "Cascadia Code".into();
        }
        if run == "\n" {
            x = area.x;
            y += line_h;
            continue;
        }
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
            let w = canvas.measure_text(word, &st).width;
            if x + w > area.right() + 0.5 && x > area.x {
                x = area.x;
                y += line_h;
            }
            placed.push((word.to_string(), st.clone(), x, y, w));
            x += w;
        }
    }
    let total = if placed.is_empty() { 0.0 } else { y + line_h - area.y };
    let dy = match style.v_align {
        TextAlign::Leading | TextAlign::Justify => 0.0,
        TextAlign::Center => ((area.height - total) / 2.0).max(0.0),
        TextAlign::Trailing => (area.height - total).max(0.0),
    };
    for (word, st, x, y, w) in placed {
        canvas.draw_text(&word, RectF::new(x, y + dy, w + 2.0, line_h), &st);
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
        repeat_enter: bool => "Behavior";
        events: on_change;
        on_set: sync
    }

    // O `on_change` é do botão marcável; o principal é o clique.
    fn default_event(&self) -> Option<&'static str> {
        Some("on_click")
    }

    fn click(&mut self, ctx: &mut EventCtx<'_>) {
        if self.checkable {
            self.checked = !self.checked;
            self.sync("checked");
            ctx.repaint();
            ctx.fire("on_change", EventArgs::Value(self.checked.into()));
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
        crate::looks::paint_button_fill(canvas, ctx, if ctx.colors.fill.is_some() { "custom" } else { ctx.variant() });
        let mut text = ctx.text_style();
        text.h_align = TextAlign::Center;
        let mut inner = ctx.rect.deflate(style.padding);
        // "&Salvar": Alt+S clica (o & some e a letra fica sublinhada).
        let (plain, at) = crate::mnemonic::caption(&self.text);
        let label = crate::looks::button_text(ctx.look(), &plain);
        let label = label.as_ref();
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
            crate::mnemonic::draw_text(canvas, label, at, inner, &text);
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
                let left = inner.x + ((inner.width - total) / 2.0).max(0.0);
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
