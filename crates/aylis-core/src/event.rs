//! Tipos de eventos de entrada (independentes de plataforma) e o contexto de evento.
//!
//! A plataforma traduz os eventos nativos para estes tipos; o core nunca vê um evento
//! do Windows, X11, Android etc.

use crate::component::ComponentId;
use crate::geometry::PointF;
use crate::prop::PropValue;
use crate::ui::Ui;

/// Teclas modificadoras pressionadas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    /// Tecla Windows / Command.
    pub meta: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PointerButton {
    #[default]
    Primary,
    Secondary,
    Middle,
    Back,
    Forward,
}

/// Origem do ponteiro: mouse, toque ou caneta.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PointerKind {
    #[default]
    Mouse,
    Touch,
    Pen,
}

/// Evento de ponteiro (mouse e toque unificados).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointerEvent {
    /// Posição relativa ao controle que recebe o evento.
    pub position: PointF,
    pub button: PointerButton,
    pub kind: PointerKind,
    pub modifiers: Modifiers,
    /// 1 = clique simples, 2 = duplo...
    pub click_count: u8,
    /// Distingue dedos em multitoque.
    pub pointer_id: u64,
}

/// Roda do mouse ou rolagem do touchpad.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WheelEvent {
    pub position: PointF,
    /// Deslocamento em unidades lógicas.
    pub delta: PointF,
    pub modifiers: Modifiers,
    /// Veio de um dedo arrastando (ou da inércia depois dele): o conteúdo deve acompanhar
    /// o dedo 1:1, sem a aceleração da roda do mouse.
    pub touch: bool,
    /// Veio de uma roda com dentes: cada dente já vem com o passo do app
    /// ([`ScrollOptions::wheel_step`](crate::ScrollOptions)) em `delta`.
    pub lines: bool,
}

/// O passo padrão de um dente da roda (pontos lógicos).
pub const WHEEL_LINE: f32 = 100.0;

/// Como a roda rola as listas e caixas de rolagem do app ([`Ui::set_scroll_options`](crate::Ui::set_scroll_options)).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollOptions {
    /// Pontos por dente da roda (100 = o padrão, como nos navegadores).
    pub wheel_step: f32,
    /// Desliza até a posição nova em vez de saltar.
    pub smooth: bool,
    /// Duração do deslize, em ms.
    pub smooth_ms: f32,
}

impl Default for ScrollOptions {
    fn default() -> Self {
        Self { wheel_step: WHEEL_LINE, smooth: true, smooth_ms: 180.0 }
    }
}

/// Para onde o dedo deslizou (o `on_swipe`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwipeDirection {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

impl SwipeDirection {
    /// O nome no protocolo e nos eventos (`left`, `right`, `up`, `down`).
    pub fn name(self) -> &'static str {
        match self {
            SwipeDirection::Left => "left",
            SwipeDirection::Right => "right",
            SwipeDirection::Up => "up",
            SwipeDirection::Down => "down",
        }
    }
}

/// Um deslizar do dedo (decidido em 14/11): um arrasto rápido que não rolou nada no mesmo
/// eixo, ao soltar. O controle tocado recebe `on_swipe`; se não tratar, sobe para os pais.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwipeEvent {
    pub direction: SwipeDirection,
    /// Onde o dedo encostou (coordenadas do controle que recebe).
    pub start: PointF,
    /// Quanto o dedo andou do começo ao fim.
    pub distance: PointF,
    /// A velocidade ao soltar, em pontos por segundo.
    pub velocity: PointF,
}

/// Gesto de dois dedos: pinça (escala), giro e arrasto do centro.
/// O controle sob o primeiro dedo recebe; se não
/// tratar, sobe para os pais.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GestureEvent {
    pub phase: GesturePhase,
    /// O ponto entre os dois dedos (coordenadas do controle).
    pub center: PointF,
    /// Escala desde o começo do gesto (1 = igual, 2 = dedos o dobro mais afastados).
    pub scale: f32,
    /// Escala desde o evento anterior (multiplique o zoom por ela).
    pub delta_scale: f32,
    /// Giro desde o começo, em radianos (positivo no sentido horário da tela).
    pub rotation: f32,
    /// Giro desde o evento anterior.
    pub delta_rotation: f32,
    /// Quanto o centro andou desde o evento anterior.
    pub pan: PointF,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GesturePhase {
    #[default]
    Begin,
    Update,
    End,
}

/// Tecla (códigos do aylis, não do sistema).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Key {
    #[default]
    Unknown,
    Enter,
    Tab,
    Escape,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Left,
    Right,
    Up,
    Down,
    Space,
    /// F1 = `F(1)`...
    F(u8),
    /// Tecla de caractere (letra, número, símbolo), já em minúscula.
    Char(char),
    Shift,
    Control,
    Alt,
    Meta,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub modifiers: Modifiers,
    /// Repetição automática por tecla mantida pressionada.
    pub repeat: bool,
}

/// Argumentos entregues aos handlers de evento.
///
/// Um tipo só para todos os eventos simplifica o encaminhamento por nome, pela C ABI e
/// pelo protocolo remoto.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum EventArgs {
    #[default]
    None,
    Pointer(PointerEvent),
    Wheel(WheelEvent),
    Gesture(GestureEvent),
    Swipe(SwipeEvent),
    Key(KeyEvent),
    Text(String),
    Value(PropValue),
    /// O `on_intercept`/`after_intercept` de um `EventInterceptor`: o evento interceptado.
    Intercept(Box<Intercepted>),
}

/// Um evento que passa por um `EventInterceptor` (veja `intercept.rs`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Intercepted {
    /// O form do componente que disparou.
    pub form: Option<crate::component::ComponentId>,
    /// O componente que disparou.
    pub sender: Option<crate::component::ComponentId>,
    /// O nome do evento (`on_click`, `before_post`...).
    pub event: String,
    /// O handler ligado ao evento (vazio: nenhum, ou uma closure em Rust).
    pub handler: String,
    /// Os argumentos do evento.
    pub args: EventArgs,
}

impl EventArgs {
    /// O texto do evento, venha como `Text` (ComboBox, ListBox) ou como valor texto
    /// (`Edit`, que manda o `text` novo como `Value`). Quem trata um `on_change` de texto
    /// usa isto e não depende de qual dos dois o controle manda.
    pub fn text(&self) -> Option<&str> {
        match self {
            EventArgs::Text(t) => Some(t),
            EventArgs::Value(PropValue::String(t)) => Some(t),
            _ => None,
        }
    }
}

/// Mensagem ([`Ui::send_message`](crate::Ui::send_message)) que o `Ui` manda ao menu de
/// contexto de um controle (a propriedade `popup_menu`) no clique direito: o menu se abre
/// em `at` (coordenadas do form). `source` é o controle clicado.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopupRequest {
    pub at: crate::geometry::PointF,
    pub source: crate::component::ComponentId,
}

/// Contexto entregue aos métodos de evento de um componente.
///
/// Enquanto o método roda, o componente está "fora" do `Ui` (por isso `ui.get(ctx.id)`
/// devolve `None` ali dentro). Os eventos disparados com [`fire`](Self::fire) são
/// entregues aos handlers depois que o método termina, quando o componente já voltou —
/// assim um handler pode alterar o próprio componente que disparou o evento.
pub struct EventCtx<'a> {
    pub ui: &'a mut Ui,
    pub id: ComponentId,
    pub(crate) handled: bool,
}

impl<'a> EventCtx<'a> {
    pub fn new(ui: &'a mut Ui, id: ComponentId) -> Self {
        Self { ui, id, handled: false }
    }

    /// Agenda o evento `event` (por exemplo `"on_click"`) deste componente.
    pub fn fire(&mut self, event: &str, args: EventArgs) {
        self.ui.queue_event(self.id, event, args);
    }

    /// Pede uma nova pintura. Chame quando o método mudar algo visual do próprio
    /// componente (alterações feitas por `ctx.ui` já pedem sozinhas). Só a área dele é
    /// repintada ([`Ui::damage`](crate::Ui::damage)); se o desenho mudou fora dela, use
    /// `ctx.ui.request_repaint()`.
    pub fn repaint(&mut self) {
        self.ui.damage(self.id);
    }

    /// Só a parte `rect` do componente mudou (nas coordenadas dele): a barra de rolagem, o
    /// cursor. Veja [`Ui::damage_part`](crate::Ui::damage_part).
    pub fn repaint_part(&mut self, rect: crate::RectF) {
        self.ui.damage_part(self.id, rect);
    }

    /// O conteúdo de `rect` (nas coordenadas do componente) andou `delta` (a rolagem): a
    /// plataforma copia os pixels já pintados e só a faixa que apareceu é pintada. Veja
    /// [`Ui::scroll_damage`](crate::Ui::scroll_damage).
    pub fn scrolled(&mut self, rect: crate::RectF, delta: crate::PointF) {
        self.ui.scroll_damage(self.id, rect, delta);
    }

    /// Pede novo layout e nova pintura (quando o método mudou tamanho, posição,
    /// alinhamento... do próprio componente).
    pub fn invalidate(&mut self) {
        self.ui.invalidate_form(self.id);
    }

    /// Muda o foco para `id` assim que este método terminar. Use dentro de métodos de
    /// evento: enquanto o método roda, o próprio componente está fora do `Ui` e não pode
    /// ganhar nem perder o foco na hora.
    pub fn request_focus(&mut self, id: ComponentId) {
        self.ui.pending_focus = Some(id);
    }

    /// Marca o evento como tratado (não propaga para o pai).
    pub fn set_handled(&mut self) {
        self.handled = true;
    }

    pub fn handled(&self) -> bool {
        self.handled
    }
}
